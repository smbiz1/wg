"""
Production Security Middleware for FastAPI
Implements high-priority security features: rate limiting, security headers, CSRF protection
"""

import time
import hmac
import hashlib
import secrets
from typing import Dict, List, Optional, Set
from fastapi import FastAPI, Request, Response, HTTPException, Depends
from fastapi.middleware.base import BaseHTTPMiddleware
from starlette.responses import JSONResponse
import redis
from contextlib import asynccontextmanager


# Rate limiting storage (Redis with local fallback)
class RateLimitStore:
    def __init__(self, redis_url: Optional[str] = None):
        self.redis_client = None
        self.local_cache: Dict[str, Dict] = {}

        if redis_url:
            try:
                self.redis_client = redis.from_url(redis_url)
                self.redis_client.ping()
            except:
                self.redis_client = None

    async def get_count(self, key: str) -> int:
        if self.redis_client:
            try:
                count = self.redis_client.get(key)
                return int(count) if count else 0
            except:
                pass

        # Local fallback
        now = time.time()
        entry = self.local_cache.get(key, {"count": 0, "window_start": now})

        # Reset if window expired
        if now - entry["window_start"] > 60:
            entry = {"count": 0, "window_start": now}
            self.local_cache[key] = entry

        return entry["count"]

    async def increment(self, key: str, window: int = 60) -> int:
        if self.redis_client:
            try:
                pipe = self.redis_client.pipeline()
                pipe.incr(key)
                pipe.expire(key, window)
                result = pipe.execute()
                return result[0]
            except:
                pass

        # Local fallback
        now = time.time()
        entry = self.local_cache.get(key, {"count": 0, "window_start": now})

        if now - entry["window_start"] > window:
            entry = {"count": 1, "window_start": now}
        else:
            entry["count"] += 1

        self.local_cache[key] = entry

        # Cleanup old entries
        self._cleanup_local_cache()

        return entry["count"]

    def _cleanup_local_cache(self):
        """Remove expired entries from local cache"""
        now = time.time()
        expired_keys = [
            key
            for key, entry in self.local_cache.items()
            if now - entry["window_start"] > 60
        ]
        for key in expired_keys:
            del self.local_cache[key]


class SecurityHeadersMiddleware(BaseHTTPMiddleware):
    """Add comprehensive security headers"""

    async def dispatch(self, request: Request, call_next):
        response = await call_next(request)

        # Essential security headers
        response.headers["X-Content-Type-Options"] = "nosniff"
        response.headers["X-Frame-Options"] = "DENY"
        response.headers["X-XSS-Protection"] = "1; mode=block"
        response.headers["Referrer-Policy"] = "strict-origin-when-cross-origin"
        response.headers["X-Security-Version"] = "Shield-Backend-v1"

        # HTTPS enforcement
        if request.headers.get("x-forwarded-proto") == "https":
            response.headers["Strict-Transport-Security"] = (
                "max-age=31536000; includeSubDomains"
            )

        # Content Security Policy (basic)
        csp = "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'"
        response.headers["Content-Security-Policy"] = csp

        return response


class CSRFMiddleware(BaseHTTPMiddleware):
    """CSRF protection for state-changing operations"""

    def __init__(self, app, secret_key: str, exempt_paths: Optional[Set[str]] = None):
        super().__init__(app)
        self.secret_key = secret_key
        self.exempt_paths = exempt_paths or {"/docs", "/openapi.json", "/health"}

    async def dispatch(self, request: Request, call_next):
        # Skip GET, HEAD, OPTIONS and exempt paths
        if (
            request.method in ["GET", "HEAD", "OPTIONS"]
            or request.url.path in self.exempt_paths
        ):
            return await call_next(request)

        # Verify CSRF token for state-changing operations
        csrf_token = request.headers.get("X-CSRF-Token")
        if not csrf_token or not self._verify_csrf_token(csrf_token):
            return JSONResponse(
                {"error": "CSRF token missing or invalid"}, status_code=403
            )

        return await call_next(request)

    def _verify_csrf_token(self, token: str) -> bool:
        """Verify CSRF token using HMAC"""
        try:
            # Extract timestamp and signature
            parts = token.split(".")
            if len(parts) != 2:
                return False

            timestamp_str, signature = parts
            timestamp = int(timestamp_str)

            # Check if token is not too old (1 hour)
            if time.time() - timestamp > 3600:
                return False

            # Verify signature
            expected_sig = hmac.new(
                self.secret_key.encode(), timestamp_str.encode(), hashlib.sha256
            ).hexdigest()

            return hmac.compare_digest(signature, expected_sig)
        except:
            return False

    def generate_csrf_token(self) -> str:
        """Generate CSRF token"""
        timestamp = str(int(time.time()))
        signature = hmac.new(
            self.secret_key.encode(), timestamp.encode(), hashlib.sha256
        ).hexdigest()
        return f"{timestamp}.{signature}"


class EnhancedRateLimitMiddleware(BaseHTTPMiddleware):
    """Enhanced rate limiting with Redis backend and configurable limits"""

    def __init__(
        self, app, store: RateLimitStore, limits: Optional[Dict[str, int]] = None
    ):
        super().__init__(app)
        self.store = store
        self.limits = limits or {
            "default": 60,  # 60 requests per minute
            "/api/scan": 10,  # 10 scans per minute
            "/api/upload": 5,  # 5 uploads per minute
        }

    async def dispatch(self, request: Request, call_next):
        # Get client IP
        client_ip = (
            request.headers.get("cf-connecting-ip")
            or request.headers.get("x-forwarded-for")
            or request.headers.get("x-real-ip")
            or request.client.host
        )

        # Determine rate limit for this endpoint
        endpoint_limit = self._get_endpoint_limit(request.url.path)

        # Check rate limit
        rate_key = f"rate:{client_ip}:{request.url.path}"
        current_count = await self.store.increment(rate_key)

        if current_count > endpoint_limit:
            return JSONResponse(
                {
                    "error": "Rate limit exceeded",
                    "limit": endpoint_limit,
                    "window": "60 seconds",
                    "retry_after": 60,
                },
                status_code=429,
                headers={"Retry-After": "60"},
            )

        # Add rate limit headers
        response = await call_next(request)
        response.headers["X-RateLimit-Limit"] = str(endpoint_limit)
        response.headers["X-RateLimit-Remaining"] = str(
            max(0, endpoint_limit - current_count)
        )
        response.headers["X-RateLimit-Reset"] = str(int(time.time()) + 60)

        return response

    def _get_endpoint_limit(self, path: str) -> int:
        """Get rate limit for specific endpoint"""
        for endpoint, limit in self.limits.items():
            if endpoint != "default" and path.startswith(endpoint):
                return limit
        return self.limits["default"]


class RequestValidationMiddleware(BaseHTTPMiddleware):
    """Basic request validation"""

    def __init__(self, app, max_content_length: int = 10 * 1024 * 1024):  # 10MB
        super().__init__(app)
        self.max_content_length = max_content_length

    async def dispatch(self, request: Request, call_next):
        # Content length validation
        content_length = int(request.headers.get("content-length", 0))
        if content_length > self.max_content_length:
            return JSONResponse({"error": "Request too large"}, status_code=413)

        # Content type validation for POST requests
        if request.method == "POST":
            content_type = request.headers.get("content-type", "")
            allowed_types = [
                "application/json",
                "application/x-www-form-urlencoded",
                "multipart/form-data",
                "text/plain",
            ]

            if not any(ct in content_type for ct in allowed_types):
                return JSONResponse(
                    {"error": "Unsupported content type"}, status_code=415
                )

        return await call_next(request)


def setup_security_middleware(
    app: FastAPI,
    redis_url: Optional[str] = None,
    csrf_secret: Optional[str] = None,
    custom_rate_limits: Optional[Dict[str, int]] = None,
):
    """Setup all security middleware for FastAPI app"""

    # Initialize rate limit store
    rate_store = RateLimitStore(redis_url)

    # Generate CSRF secret if not provided
    if not csrf_secret:
        csrf_secret = secrets.token_urlsafe(32)

    # Add middleware in reverse order (last added = first executed)
    app.add_middleware(RequestValidationMiddleware)
    app.add_middleware(
        EnhancedRateLimitMiddleware, store=rate_store, limits=custom_rate_limits
    )
    app.add_middleware(CSRFMiddleware, secret_key=csrf_secret)
    app.add_middleware(SecurityHeadersMiddleware)

    # Store CSRF middleware reference for token generation
    app.state.csrf_secret = csrf_secret

    return app


def get_csrf_token(request: Request) -> str:
    """Generate CSRF token for current request"""
    csrf_secret = request.app.state.csrf_secret
    timestamp = str(int(time.time()))
    signature = hmac.new(
        csrf_secret.encode(), timestamp.encode(), hashlib.sha256
    ).hexdigest()
    return f"{timestamp}.{signature}"


# Dependency for endpoints that need CSRF tokens
async def csrf_token_dependency(request: Request) -> str:
    """FastAPI dependency to provide CSRF token"""
    return get_csrf_token(request)


# Usage example:
"""
from fastapi import FastAPI, Depends
from .security_middleware import setup_security_middleware, csrf_token_dependency

app = FastAPI()

# Setup security middleware
setup_security_middleware(
    app,
    redis_url="redis://localhost:6379",
    custom_rate_limits={
        "/api/scan": 5,  # 5 scans per minute
        "/api/upload": 3,  # 3 uploads per minute
    }
)

@app.get("/csrf-token")
async def get_csrf(token: str = Depends(csrf_token_dependency)):
    return {"csrf_token": token}

@app.post("/api/protected")
async def protected_endpoint(data: dict):
    return {"status": "success"}
"""
