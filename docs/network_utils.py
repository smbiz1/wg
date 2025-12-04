network_utils.py


def validate_ip_address(ip: str) -> bool:
    """Validate if the given string is a valid IP address."""
    try:
        ipaddress.ip_address(ip)
        return True
    except ValueError:
        return False


def is_internal_ip(ip: str) -> bool:
    """Check if IP address is internal/private."""
    try:
        ip_obj = ipaddress.ip_address(ip)
        return ip_obj.is_private
    except ValueError:
        return False


def validate_hostname(hostname: str) -> bool:
    """Validate hostname format according to RFC 1123."""
    if not hostname or len(hostname) > 253:
        return False

    # RFC 1123 compliant hostname validation
    pattern = r"^[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?)*$"
    return bool(re.match(pattern, hostname))


def generate_csrf_token() -> str:
    """Generate a secure CSRF token."""
    return secrets.token_urlsafe(32)


def validate_csrf_token(token: str, stored_token: str) -> bool:
    """Validate CSRF token using constant-time comparison."""
    return secrets.compare_digest(token, stored_token)


def get_secure_headers() -> Dict[str, str]:
    """Return secure HTTP headers for server responses."""
    return {
        "X-Content-Type-Options": "nosniff",
        "X-Frame-Options": "DENY",
        "X-XSS-Protection": "1; mode=block",
        "Strict-Transport-Security": "max-age=31536000; includeSubDomains",
        "Content-Security-Policy": "default-src 'self'; script-src 'self' 'unsafe-inline'",
        "Referrer-Policy": "strict-origin-when-cross-origin",
    }


def sanitize_http_headers(headers: Dict[str, str]) -> Dict[str, str]:
    """Sanitize HTTP headers to remove potentially dangerous values."""
    sanitized = {}
    safe_headers = {"content-type", "accept", "user-agent", "authorization"}

    for key, value in headers.items():
        key_lower = key.lower()
        if key_lower.startswith("x-") or key_lower in safe_headers:
            sanitized[key] = sanitize_input(value, 1000)

    return sanitized


def validate_redirect_url(url: str, allowed_domains: List[str]) -> bool:
    """Validate redirect URL to prevent open redirect vulnerabilities."""
    try:
        parsed = urlparse(url)

        # Relative URLs are safe
        if not parsed.netloc:
            return True

        # Extract domain without port
        domain = parsed.netloc.split(":")[0].lower()

        # Check against allowed domains
        return domain in allowed_domains or any(
            domain.endswith(f".{d}") for d in allowed_domains
        )
    except Exception:
        return False


def rate_limit_key(client_ip: str, endpoint: str) -> str:
    """Generate a rate limiting key based on client IP and endpoint."""
    return f"rate_limit:{client_ip}:{endpoint}"
