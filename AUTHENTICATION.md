# Authentication System for Isobox

Isobox now includes a comprehensive authentication system that supports multiple authentication strategies, caching, and code deduplication. This document provides a complete guide to configuring and using the authentication features.

## Table of Contents

1. [Overview](#overview)
2. [Authentication Strategies](#authentication-strategies)
3. [Configuration](#configuration)
4. [API Endpoints](#api-endpoints)
5. [Code Deduplication](#code-deduplication)
6. [Security Considerations](#security-considerations)
7. [Examples](#examples)
8. [Troubleshooting](#troubleshooting)

## Overview

The authentication system in Isobox provides:

- **Multiple Authentication Strategies**: JWT, OAuth2, API Keys, mTLS, and no authentication
- **Flexible Configuration**: Environment-based configuration with sensible defaults
- **Caching Support**: Redis and in-memory caching for authentication results
- **Code Deduplication**: Hash-based caching to prevent duplicate code execution
- **Permission-Based Access Control**: Fine-grained permission system
- **Middleware Integration**: Seamless integration with Actix-Web

## Authentication Strategies

### 1. No Authentication (Default)

The simplest strategy that allows all requests without authentication.

```bash
AUTH_TYPE=none
```

### 2. JWT Authentication

Supports JWT tokens from various providers (Google, Auth0, etc.).

```bash
AUTH_TYPE=jwt
JWT_ISSUER_URL=https://accounts.google.com
JWT_AUDIENCE=your-app-id
JWT_PUBLIC_KEY_URL=https://www.googleapis.com/oauth2/v1/certs
JWT_CACHE_TTL=3600
```

**Usage:**

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"language": "python", "code": "print(\"Hello, World!\")"}'
```

### 3. OAuth2 Authentication

Supports various OAuth2 providers: Google, Meta, GitHub, Firebase, and AWS Cognito.

```bash
AUTH_TYPE=oauth2
OAUTH2_PROVIDER=google
OAUTH2_CLIENT_ID=your-client-id
OAUTH2_CLIENT_SECRET=your-client-secret
OAUTH2_TOKEN_URL=https://oauth2.googleapis.com/token
OAUTH2_USERINFO_URL=https://www.googleapis.com/oauth2/v2/userinfo
```

**Supported Providers:**

- `google` - Google OAuth2
- `meta` - Meta/Facebook OAuth2
- `github` - GitHub OAuth2
- `firebase` - Firebase Authentication
- `cognito` - AWS Cognito

### 4. API Key Authentication

Simple API key-based authentication.

```bash
AUTH_TYPE=apikey
API_KEYS=key1,key2,key3
API_KEY_HEADER=X-API-Key
```

**Usage:**

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "X-API-Key: your-api-key" \
  -H "Content-Type: application/json" \
  -d '{"language": "python", "code": "print(\"Hello, World!\")"}'
```

### 5. Mutual TLS (mTLS) Authentication

Client certificate-based authentication.

```bash
AUTH_TYPE=mtls
MTLS_CA_CERT_PATH=/path/to/ca.crt
MTLS_CLIENT_CERT_REQUIRED=true
MTLS_VERIFY_HOSTNAME=true
```

## Configuration

### Environment Variables

All configuration is done through environment variables:

#### Authentication Type

```bash
AUTH_TYPE=none|jwt|mtls|apikey|oauth2
```

#### JWT Configuration

```bash
JWT_ISSUER_URL=https://accounts.google.com
JWT_AUDIENCE=your-app-id
JWT_PUBLIC_KEY_URL=https://www.googleapis.com/oauth2/v1/certs
JWT_CACHE_TTL=3600
```

#### OAuth2 Configuration

```bash
OAUTH2_PROVIDER=google|meta|github|firebase|cognito
OAUTH2_CLIENT_ID=your-client-id
OAUTH2_CLIENT_SECRET=your-client-secret
OAUTH2_TOKEN_URL=https://oauth2.googleapis.com/token
OAUTH2_USERINFO_URL=https://www.googleapis.com/oauth2/v2/userinfo
```

#### API Key Configuration

```bash
API_KEYS=key1,key2,key3
API_KEY_HEADER=X-API-Key
```

#### mTLS Configuration

```bash
MTLS_CA_CERT_PATH=/path/to/ca.crt
MTLS_CLIENT_CERT_REQUIRED=true
MTLS_VERIFY_HOSTNAME=true
```

#### Cache Configuration

```bash
REDIS_URL=redis://localhost:6379
AUTH_CACHE_TTL=300
MAX_CACHE_SIZE=10000
```

#### Deduplication Configuration

```bash
DEDUP_ENABLED=true
DEDUP_CACHE_TTL=3600
DEDUP_CACHE_TYPE=redis|memory
```

### Configuration Examples

#### Development (No Authentication)

```bash
AUTH_TYPE=none
DEDUP_ENABLED=false
```

#### Production with JWT

```bash
AUTH_TYPE=jwt
JWT_ISSUER_URL=https://accounts.google.com
JWT_AUDIENCE=your-app-id
JWT_PUBLIC_KEY_URL=https://www.googleapis.com/oauth2/v1/certs
DEDUP_ENABLED=true
DEDUP_CACHE_TYPE=redis
REDIS_URL=redis://redis:6379
```

#### Production with API Keys

```bash
AUTH_TYPE=apikey
API_KEYS=prod-key-1,prod-key-2,prod-key-3
DEDUP_ENABLED=true
DEDUP_CACHE_TYPE=memory
```

## API Endpoints

### Public Endpoints

#### Health Check

```bash
GET /health
```

### Authenticated Endpoints

#### Execute Code

```bash
POST /api/v1/execute
Authorization: Bearer <token> | X-API-Key: <key>
Content-Type: application/json

{
  "language": "python",
  "code": "print('Hello, World!')"
}
```

#### Authentication Status

```bash
GET /auth/status
Authorization: Bearer <token> | X-API-Key: <key>
```

### Admin Endpoints

#### Deduplication Statistics

```bash
GET /admin/dedup/stats
Authorization: Bearer <token> | X-API-Key: <key>
```

## Code Deduplication

The deduplication system prevents duplicate code execution by caching results based on a hash of the language and code combination.

### Features

- **Hash-based Detection**: SHA-256 hash of `language:code` combination
- **Configurable TTL**: Set cache expiration time
- **Multiple Backends**: Redis or in-memory storage
- **Automatic Cleanup**: Expired entries are automatically removed

### Configuration

```bash
# Enable deduplication
DEDUP_ENABLED=true

# Set cache TTL (in seconds)
DEDUP_CACHE_TTL=3600

# Choose cache backend
DEDUP_CACHE_TYPE=redis|memory
```

### Benefits

- **Performance**: Avoids redundant code execution
- **Resource Savings**: Reduces Docker container usage
- **Consistency**: Ensures identical results for identical code
- **Scalability**: Reduces load on the system

### Example

```bash
# First execution
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Authorization: Bearer token" \
  -d '{"language": "python", "code": "print(2+2)"}'
# Returns: {"stdout": "4\n", "stderr": "", "exit_code": 0, "time_taken": 0.123}

# Second execution (same code)
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Authorization: Bearer token" \
  -d '{"language": "python", "code": "print(2+2)"}'
# Returns: {"stdout": "4\n", "stderr": "", "exit_code": 0, "time_taken": 0.123}
# (Cached result, no actual execution)
```

## Security Considerations

### JWT Security

- Verify token signature with proper public keys
- Validate issuer, audience, and expiration
- Use secure algorithms (RS256, ES256)
- Implement token blacklisting for revoked tokens

### OAuth2 Security

- Validate tokens with the provider
- Check token expiration and scope
- Verify client credentials
- Use HTTPS for all communications

### API Key Security

- Use strong, random API keys
- Rotate keys regularly
- Store keys securely
- Use HTTPS for all communications

### mTLS Security

- Validate client certificates against CA
- Check certificate revocation lists (CRL)
- Implement certificate pinning
- Use strong cipher suites

### General Security

- Use HTTPS in production
- Implement rate limiting
- Monitor authentication attempts
- Log security events
- Keep dependencies updated

## Examples

### Complete Docker Compose Setup

```yaml
version: "3.8"

services:
  isobox:
    build: .
    ports:
      - "8000:8000"
    environment:
      - AUTH_TYPE=jwt
      - JWT_ISSUER_URL=https://accounts.google.com
      - JWT_AUDIENCE=your-app-id
      - JWT_PUBLIC_KEY_URL=https://www.googleapis.com/oauth2/v1/certs
      - DEDUP_ENABLED=true
      - DEDUP_CACHE_TYPE=redis
      - REDIS_URL=redis://redis:6379
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
    depends_on:
      - redis

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

volumes:
  redis_data:
```

### Python Client Example

```python
import requests
import json

class IsoboxClient:
    def __init__(self, base_url, auth_token=None, api_key=None):
        self.base_url = base_url
        self.headers = {'Content-Type': 'application/json'}

        if auth_token:
            self.headers['Authorization'] = f'Bearer {auth_token}'
        elif api_key:
            self.headers['X-API-Key'] = api_key

    def execute_code(self, language, code):
        url = f"{self.base_url}/api/v1/execute"
        data = {
            'language': language,
            'code': code
        }

        response = requests.post(url, headers=self.headers, json=data)
        response.raise_for_status()
        return response.json()

    def get_auth_status(self):
        url = f"{self.base_url}/auth/status"
        response = requests.get(url, headers=self.headers)
        response.raise_for_status()
        return response.json()

# Usage
client = IsoboxClient(
    'http://localhost:8000',
    auth_token='your-jwt-token'
)

result = client.execute_code('python', 'print("Hello, World!")')
print(result)
```

### JavaScript Client Example

```javascript
class IsoboxClient {
  constructor(baseUrl, authToken = null, apiKey = null) {
    this.baseUrl = baseUrl;
    this.headers = {
      "Content-Type": "application/json",
    };

    if (authToken) {
      this.headers["Authorization"] = `Bearer ${authToken}`;
    } else if (apiKey) {
      this.headers["X-API-Key"] = apiKey;
    }
  }

  async executeCode(language, code) {
    const url = `${this.baseUrl}/api/v1/execute`;
    const data = {
      language,
      code,
    };

    const response = await fetch(url, {
      method: "POST",
      headers: this.headers,
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json();
  }

  async getAuthStatus() {
    const url = `${this.baseUrl}/auth/status`;
    const response = await fetch(url, {
      headers: this.headers,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json();
  }
}

// Usage
const client = new IsoboxClient("http://localhost:8000", "your-jwt-token");

client
  .executeCode("python", 'print("Hello, World!")')
  .then((result) => console.log(result))
  .catch((error) => console.error(error));
```

## Troubleshooting

### Common Issues

#### Authentication Errors

**Error: "Authentication required"**

- Check if `AUTH_TYPE` is set correctly
- Verify credentials are provided in the request
- Ensure the authentication strategy is properly configured

**Error: "Invalid token"**

- Verify token format and signature
- Check token expiration
- Ensure issuer and audience match configuration

**Error: "Configuration error"**

- Check all required environment variables are set
- Verify configuration values are valid
- Check file paths for mTLS certificates

#### Deduplication Issues

**Deduplication not working**

- Ensure `DEDUP_ENABLED=true`
- Check cache backend configuration
- Verify Redis connection (if using Redis)

**Cache connection errors**

- Check Redis URL and connectivity
- Verify Redis server is running
- Check network connectivity

#### Performance Issues

**Slow authentication**

- Enable authentication caching
- Use Redis for better performance
- Check network latency to external providers

**High memory usage**

- Reduce cache TTL values
- Use Redis instead of memory cache
- Implement cache size limits

### Debugging

Enable debug logging:

```bash
RUST_LOG=debug cargo run
```

Check authentication status:

```bash
curl -H "Authorization: Bearer token" http://localhost:8000/auth/status
```

Check deduplication stats:

```bash
curl -H "Authorization: Bearer token" http://localhost:8000/admin/dedup/stats
```

### Monitoring

Monitor these metrics:

- Authentication success/failure rates
- Cache hit rates for deduplication
- Response times
- Error rates by authentication type
- Memory usage
- Redis connection status

## Migration Guide

### From No Authentication

1. Set `AUTH_TYPE` to your desired strategy
2. Configure the required environment variables
3. Update client code to include authentication headers
4. Test with a small subset of requests

### From API Keys to JWT

1. Set `AUTH_TYPE=jwt`
2. Configure JWT environment variables
3. Update client code to use Bearer tokens
4. Remove API key headers from requests

### Enabling Deduplication

1. Set `DEDUP_ENABLED=true`
2. Choose cache backend (`memory` or `redis`)
3. Set appropriate TTL values
4. Monitor cache performance

## Support

For issues and questions:

1. Check the troubleshooting section
2. Review configuration examples
3. Enable debug logging
4. Check the application logs
5. Verify environment variables

The authentication system is designed to be flexible and secure while maintaining high performance and ease of use.
