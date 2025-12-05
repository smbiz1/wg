# src/lib/ai/network_manager.py
import os
import json
import subprocess
import logging
import asyncio
from typing import Optional

logger = logging.getLogger(__name__)

class NetworkManager:
    """Manages network connectivity for secure AI access"""
    
    def __init__(self):
        self.tailscale_status = None
        self.tailscale_configured = False
    
    async def ensure_tailscale(self) -> bool:
        """Ensure Tailscale is configured and running"""
        # Skip if already confirmed running
        if self.tailscale_configured:
            return True
            
        # Check if already running
        if await self._check_tailscale_status():
            self.tailscale_configured = True
            return True
        
        # Not running, try to start it
        await self._setup_tailscale()
        
        # Check again
        if await self._check_tailscale_status():
            self.tailscale_configured = True
            return True
            
        return False
    
    async def _check_tailscale_status(self) -> bool:
        """Check if Tailscale is running"""
        try:
            result = await asyncio.create_subprocess_exec(
                "tailscale", "status", "--json",
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE
            )
            stdout, stderr = await result.communicate()
            
            if result.returncode == 0:
                self.tailscale_status = json.loads(stdout.decode())
                return True
            return False
        except Exception as e:
            logger.error(f"Tailscale status check failed: {str(e)}")
            return False
    
    async def _setup_tailscale(self) -> bool:
        """Configure and start Tailscale"""
        try:
            # Get auth key from environment or path
            auth_key = self._get_tailscale_auth_key()
            if not auth_key:
                logger.error("No Tailscale auth key available")
                return False
            
            # Start Tailscale
            cmd = [
                "tailscale", "up",
                "--authkey", auth_key,
                "--hostname", "ai-router",
                "--accept-routes",
                "--shields-up"
            ]
            
            process = await asyncio.create_subprocess_exec(
                *cmd,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE
            )
            stdout, stderr = await process.communicate()
            
            if process.returncode != 0:
                logger.error(f"Failed to start Tailscale: {stderr.decode()}")
                return False
                
            logger.info("Tailscale started successfully")
            return True
            
        except Exception as e:
            logger.error(f"Error setting up Tailscale: {str(e)}")
            return False
    
    def _get_tailscale_auth_key(self) -> Optional[str]:
        """Get Tailscale auth key from environment or file"""
        # First check environment
        auth_key = os.getenv("TAILSCALE_AUTH_KEY")
        if auth_key:
            return auth_key
            
        # Try file path
        auth_key_path = os.getenv("TAILSCALE_AUTH_KEY_PATH", "/run/secrets/tailscale/authkey")
        try:
            if os.path.exists(auth_key_path):
                with open(auth_key_path, 'r') as f:
                    return f.read().strip()
        except Exception as e:
            logger.error(f"Error reading Tailscale auth key file: {str(e)}")
            
        return None