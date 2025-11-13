#!/usr/bin/env python3
"""
VPN Provisioning API

This API handles:
1. API key validation (integrate with your existing user authentication)
2. WireGuard client configuration generation
3. Server list provision based on subscription tier
4. Client lifecycle management (revocation, etc.)

Usage:
    python3 provision_api.py

API Endpoints:
    POST /api/provision - Generate WireGuard config for a user
    GET /api/servers - Get available servers for a user
    POST /api/revoke - Revoke a client's access
"""

from flask import Flask, request, jsonify
from flask_cors import CORS
import json
import subprocess
import os
import secrets
import hashlib
import hmac
from datetime import datetime, timedelta
from pathlib import Path
import logging

app = Flask(__name__)
CORS(app)  # Enable CORS for desktop app

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Paths
SCRIPT_DIR = Path(__file__).parent
SERVERS_FILE = SCRIPT_DIR / "servers.json"
WG_INSTALL_SCRIPT = Path(__file__).parent.parent.parent / "wireguard-install.sh"
CLIENT_CONFIGS_DIR = Path("/etc/wireguard/clients")

# TODO: Replace with your actual secret key management system
# This should be stored in environment variables or a secure secret manager
API_SECRET_KEY = os.environ.get("VPN_API_SECRET_KEY", "CHANGE_THIS_SECRET_KEY")

# Load server configuration
with open(SERVERS_FILE) as f:
    SERVER_CONFIG = json.load(f)


def validate_api_key(api_key):
    """
    Validate API key and return user information.

    TODO: Replace this with your actual authentication system.
    This should integrate with your existing user database/authentication.

    Expected API key format (example):
        base64(user_id:tier:expiry:signature)

    Returns:
        dict: User information if valid, None if invalid
        {
            'user_id': str,
            'tier': str,  # 'basic' or 'pro'
            'email': str,
            'expires_at': datetime
        }
    """
    if not api_key:
        return None

    try:
        # EXAMPLE IMPLEMENTATION - Replace with your actual auth
        # For demo purposes, we'll accept keys in format: userid_tier_signature
        # In production, this should validate against your user database

        # Simple validation example (REPLACE THIS)
        parts = api_key.split('_')
        if len(parts) >= 3:
            user_id = parts[0]
            tier = parts[1]
            provided_signature = parts[2]

            # Verify signature (HMAC)
            expected_signature = hmac.new(
                API_SECRET_KEY.encode(),
                f"{user_id}:{tier}".encode(),
                hashlib.sha256
            ).hexdigest()[:16]

            if provided_signature == expected_signature:
                # TODO: Query your database to get actual user info
                return {
                    'user_id': user_id,
                    'tier': tier if tier in ['basic', 'pro'] else 'basic',
                    'email': f"{user_id}@example.com",  # Replace with actual email
                    'expires_at': datetime.now() + timedelta(days=30)
                }

        logger.warning(f"Invalid API key format: {api_key[:10]}...")
        return None

    except Exception as e:
        logger.error(f"API key validation error: {e}")
        return None


def get_available_servers(tier):
    """
    Get list of servers available for a given subscription tier.

    Args:
        tier (str): User's subscription tier ('basic' or 'pro')

    Returns:
        list: Available servers for the tier
    """
    all_servers = SERVER_CONFIG['servers']
    tier_config = SERVER_CONFIG['tiers'].get(tier, SERVER_CONFIG['tiers']['basic'])
    max_servers = tier_config['max_servers']

    # Filter servers: basic tier gets 'basic' servers, pro gets all
    if tier == 'basic':
        available = [s for s in all_servers if s.get('tier') == 'basic']
    else:
        available = all_servers

    return available[:max_servers]


def generate_client_config(user_id, server_id):
    """
    Generate WireGuard client configuration for a user.

    This uses the existing wireguard-install.sh script to generate configs,
    ensuring we don't duplicate key generation logic.

    Args:
        user_id (str): Unique user identifier
        server_id (str): Server ID from servers.json

    Returns:
        dict: Client configuration or None if failed
        {
            'config': str,  # WireGuard config file contents
            'client_name': str,
            'server': dict  # Server information
        }
    """
    try:
        # Find server configuration
        server = next(
            (s for s in SERVER_CONFIG['servers'] if s['id'] == server_id),
            None
        )

        if not server:
            logger.error(f"Server not found: {server_id}")
            return None

        # Generate unique client name
        client_name = f"{user_id}_{server_id}_{secrets.token_hex(4)}"

        # Check if WireGuard is installed and configured on this server
        if not WG_INSTALL_SCRIPT.exists():
            logger.error(f"WireGuard install script not found at {WG_INSTALL_SCRIPT}")
            return None

        # Use wireguard-install.sh to create client
        # NOTE: This assumes the script is running on a configured WireGuard server
        cmd = [
            'bash',
            str(WG_INSTALL_SCRIPT),
            '--addclient',
            client_name
        ]

        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=30
        )

        if result.returncode != 0:
            logger.error(f"Failed to create client: {result.stderr}")
            return None

        # Read generated client config
        # The script creates configs in the home directory or /root
        possible_paths = [
            Path(f"/root/{client_name}.conf"),
            Path(f"/home/{os.environ.get('SUDO_USER', 'root')}/{client_name}.conf"),
            CLIENT_CONFIGS_DIR / f"{client_name}.conf"
        ]

        config_content = None
        for config_path in possible_paths:
            if config_path.exists():
                config_content = config_path.read_text()
                logger.info(f"Found config at {config_path}")
                break

        if not config_content:
            logger.error(f"Client config file not found for {client_name}")
            return None

        return {
            'config': config_content,
            'client_name': client_name,
            'server': server
        }

    except subprocess.TimeoutExpired:
        logger.error("WireGuard client creation timed out")
        return None
    except Exception as e:
        logger.error(f"Error generating client config: {e}")
        return None


@app.route('/api/health', methods=['GET'])
def health_check():
    """Health check endpoint."""
    return jsonify({
        'status': 'healthy',
        'timestamp': datetime.now().isoformat()
    })


@app.route('/api/servers', methods=['GET'])
def list_servers():
    """
    Get available servers for the authenticated user.

    Headers:
        X-API-Key: User's API key

    Returns:
        200: List of available servers
        401: Invalid/missing API key
    """
    api_key = request.headers.get('X-API-Key')

    user = validate_api_key(api_key)
    if not user:
        return jsonify({'error': 'Invalid or missing API key'}), 401

    servers = get_available_servers(user['tier'])

    return jsonify({
        'servers': servers,
        'tier': user['tier'],
        'tier_info': SERVER_CONFIG['tiers'][user['tier']]
    })


@app.route('/api/provision', methods=['POST'])
def provision_client():
    """
    Provision a new WireGuard client configuration.

    Headers:
        X-API-Key: User's API key

    Body:
        {
            "server_id": "us-east-1"  # Optional, auto-selected if not provided
        }

    Returns:
        200: WireGuard configuration
        400: Invalid request
        401: Invalid API key
        403: Server not available for user's tier
        500: Provisioning failed
    """
    api_key = request.headers.get('X-API-Key')

    user = validate_api_key(api_key)
    if not user:
        return jsonify({'error': 'Invalid or missing API key'}), 401

    data = request.get_json() or {}
    server_id = data.get('server_id')

    # Get available servers for user's tier
    available_servers = get_available_servers(user['tier'])

    # If no server specified, use first available (client will auto-select by latency)
    if not server_id:
        if not available_servers:
            return jsonify({'error': 'No servers available'}), 500
        server_id = available_servers[0]['id']

    # Verify server is available for user's tier
    if not any(s['id'] == server_id for s in available_servers):
        return jsonify({
            'error': 'Server not available for your subscription tier'
        }), 403

    # Generate client configuration
    logger.info(f"Provisioning client for user {user['user_id']} on server {server_id}")

    client_config = generate_client_config(user['user_id'], server_id)

    if not client_config:
        return jsonify({'error': 'Failed to generate client configuration'}), 500

    return jsonify({
        'success': True,
        'config': client_config['config'],
        'client_name': client_config['client_name'],
        'server': client_config['server'],
        'available_servers': available_servers
    })


@app.route('/api/revoke', methods=['POST'])
def revoke_client():
    """
    Revoke a client's WireGuard access.

    Headers:
        X-API-Key: User's API key

    Body:
        {
            "client_name": "userid_serverid_abc123"
        }

    Returns:
        200: Client revoked successfully
        401: Invalid API key
        404: Client not found
        500: Revocation failed
    """
    api_key = request.headers.get('X-API-Key')

    user = validate_api_key(api_key)
    if not user:
        return jsonify({'error': 'Invalid or missing API key'}), 401

    data = request.get_json() or {}
    client_name = data.get('client_name')

    if not client_name:
        return jsonify({'error': 'client_name is required'}), 400

    # Verify client belongs to this user
    if not client_name.startswith(f"{user['user_id']}_"):
        return jsonify({'error': 'Unauthorized to revoke this client'}), 403

    try:
        # Use wireguard-install.sh to remove client
        cmd = [
            'bash',
            str(WG_INSTALL_SCRIPT),
            '--removeclient',
            client_name,
            '-y'  # Auto-confirm
        ]

        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=30
        )

        if result.returncode != 0:
            logger.error(f"Failed to revoke client: {result.stderr}")
            return jsonify({'error': 'Failed to revoke client'}), 500

        logger.info(f"Revoked client {client_name} for user {user['user_id']}")

        return jsonify({
            'success': True,
            'message': f'Client {client_name} revoked successfully'
        })

    except subprocess.TimeoutExpired:
        return jsonify({'error': 'Revocation timed out'}), 500
    except Exception as e:
        logger.error(f"Error revoking client: {e}")
        return jsonify({'error': 'Internal server error'}), 500


@app.route('/api/generate-api-key', methods=['POST'])
def generate_api_key():
    """
    Generate API key for a user (for testing/development).

    TODO: Remove this in production - API keys should be generated
    through your main application's authentication flow.

    Body:
        {
            "user_id": "user123",
            "tier": "basic" or "pro"
        }

    Returns:
        200: Generated API key
    """
    data = request.get_json() or {}
    user_id = data.get('user_id', 'demo_user')
    tier = data.get('tier', 'basic')

    if tier not in ['basic', 'pro']:
        tier = 'basic'

    # Generate signature
    signature = hmac.new(
        API_SECRET_KEY.encode(),
        f"{user_id}:{tier}".encode(),
        hashlib.sha256
    ).hexdigest()[:16]

    api_key = f"{user_id}_{tier}_{signature}"

    return jsonify({
        'api_key': api_key,
        'user_id': user_id,
        'tier': tier,
        'note': 'Save this API key - it will be used in the desktop app'
    })


if __name__ == '__main__':
    # Ensure client configs directory exists
    CLIENT_CONFIGS_DIR.mkdir(parents=True, exist_ok=True)

    # Print startup information
    logger.info("=" * 60)
    logger.info("VPN Provisioning API Starting")
    logger.info("=" * 60)
    logger.info(f"Servers config: {SERVERS_FILE}")
    logger.info(f"WireGuard script: {WG_INSTALL_SCRIPT}")
    logger.info(f"Client configs dir: {CLIENT_CONFIGS_DIR}")
    logger.info("")
    logger.info("IMPORTANT: Before running in production:")
    logger.info("1. Replace validate_api_key() with your auth system")
    logger.info("2. Set VPN_API_SECRET_KEY environment variable")
    logger.info("3. Remove /api/generate-api-key endpoint")
    logger.info("4. Configure HTTPS/TLS")
    logger.info("5. Set up proper logging and monitoring")
    logger.info("=" * 60)

    # Run development server
    # In production, use gunicorn or similar WSGI server
    app.run(
        host='0.0.0.0',
        port=5000,
        debug=os.environ.get('FLASK_DEBUG', 'False') == 'True'
    )
