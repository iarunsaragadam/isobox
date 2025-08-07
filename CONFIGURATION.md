# Isobox Configuration Guide

Isobox is highly configurable through environment variables. This guide explains all available configuration options for authentication, CORS, and other features.

## Quick Start Examples

### No Authentication (Development)

```bash
AUTH_TYPE=none
CORS_ENABLED=true
CORS_ALLOWED_ORIGINS=*
```

### API Key Authentication

```bash
AUTH_TYPE=apikey
API_KEYS=key1,key2,key3
API_KEY_HEADER_NAME=X-API-Key
```

### JWT Authentication (Firebase)

```bash
AUTH_TYPE=jwt
JWT_ISSUER_URL=https://securetoken.google.com/your-project-id
JWT_AUDIENCE=your-project-id
JWT_PUBLIC_KEY_URL=https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com
```

### JWT Authentication (Microsoft Azure AD)

```bash
AUTH_TYPE=jwt
JWT_ISSUER_URL=https://login.microsoftonline.com/your-tenant-id/v2.0
JWT_AUDIENCE=your-app-id
JWT_PUBLIC_KEY_URL=https://login.microsoftonline.com/your-tenant-id/discovery/v2.0/keys
```

### CORS Configuration

```bash
CORS_ENABLED=true
CORS_ALLOWED_ORIGINS=https://yourdomain.com,https://app.yourdomain.com
CORS_ALLOWED_METHODS=GET,POST,PUT,DELETE,OPTIONS
CORS_ALLOWED_HEADERS=Content-Type,Authorization,X-API-Key
CORS_ALLOW_CREDENTIALS=true
CORS_MAX_AGE=3600
```

## Authentication Configuration

### AUTH_TYPE

**Required**: `none` | `jwt` | `apikey` | `mtls` | `oauth2`

Determines the authentication method used by the service.

- `none`: No authentication required (development mode)
- `jwt`: JSON Web Token authentication
- `apikey`: API key authentication
- `mtls`: Mutual TLS authentication
- `oauth2`: OAuth 2.0 authentication

**Default**: `none`

### JWT Configuration

#### JWT_ISSUER_URL

**Required when AUTH_TYPE=jwt**

The issuer URL for JWT tokens. Examples:

- Firebase: `https://securetoken.google.com/your-project-id`
- Microsoft Azure AD: `https://login.microsoftonline.com/your-tenant-id/v2.0`
- Auth0: `https://your-domain.auth0.com/`

#### JWT_AUDIENCE

**Required when AUTH_TYPE=jwt**

The audience claim expected in JWT tokens. Examples:

- Firebase: Your Firebase project ID
- Microsoft Azure AD: Your application ID
- Auth0: Your API identifier

#### JWT_PUBLIC_KEY_URL

**Required when AUTH_TYPE=jwt**

URL to fetch the public keys for JWT verification. Examples:

- Firebase: `https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com`
- Microsoft Azure AD: `https://login.microsoftonline.com/your-tenant-id/discovery/v2.0/keys`
- Auth0: `https://your-domain.auth0.com/.well-known/jwks.json`

#### JWT_CACHE_TTL

**Optional**

Cache TTL for JWT public keys in seconds.

**Default**: `3600` (1 hour)

### API Key Configuration

#### API_KEYS

**Required when AUTH_TYPE=apikey**

Comma-separated list of valid API keys.

**Example**: `key1,key2,key3`

#### API_KEY_HEADER_NAME

**Optional**

HTTP header name for API key.

**Default**: `X-API-Key`

### mTLS Configuration

#### MTLS_CA_CERT_PATH

**Required when AUTH_TYPE=mtls**

Path to the CA certificate file for client certificate verification.

#### MTLS_CLIENT_CERT_REQUIRED

**Optional**

Whether client certificates are required.

**Default**: `true`

#### MTLS_VERIFY_HOSTNAME

**Optional**

Whether to verify the hostname in client certificates.

**Default**: `true`

### OAuth 2.0 Configuration

#### OAUTH2_PROVIDER

**Required when AUTH_TYPE=oauth2**

OAuth 2.0 provider: `google` | `meta` | `github` | `firebase` | `cognito` | `microsoft` | `custom`

#### OAUTH2_CLIENT_ID

**Required when AUTH_TYPE=oauth2**

OAuth 2.0 client ID.

#### OAUTH2_CLIENT_SECRET

**Required when AUTH_TYPE=oauth2**

OAuth 2.0 client secret.

#### OAUTH2_TOKEN_URL

**Required when AUTH_TYPE=oauth2**

OAuth 2.0 token endpoint URL.

#### OAUTH2_USERINFO_URL

**Required when AUTH_TYPE=oauth2**

OAuth 2.0 userinfo endpoint URL.

## CORS Configuration

### CORS_ENABLED

**Optional**

Enable CORS support.

**Default**: `false`

### CORS_ALLOWED_ORIGINS

**Required when CORS_ENABLED=true**

Comma-separated list of allowed origins. Use `*` to allow all origins.

**Examples**:

- `*` (allow all origins)
- `https://yourdomain.com`
- `https://yourdomain.com,https://app.yourdomain.com`

### CORS_ALLOWED_METHODS

**Optional**

Comma-separated list of allowed HTTP methods.

**Default**: `GET,POST,PUT,DELETE,OPTIONS`

### CORS_ALLOWED_HEADERS

**Optional**

Comma-separated list of allowed HTTP headers.

**Default**: `Content-Type,Authorization,X-API-Key`

### CORS_ALLOW_CREDENTIALS

**Optional**

Allow credentials (cookies, authorization headers) in CORS requests.

**Default**: `false`

### CORS_MAX_AGE

**Optional**

Maximum age for CORS preflight requests in seconds.

**Default**: Not set (browser default)

## Cache Configuration

### AUTH_CACHE_TTL

**Optional**

Authentication cache TTL in seconds.

**Default**: `3600` (1 hour)

### AUTH_CACHE_MAX_SIZE

**Optional**

Maximum number of cached authentication results.

**Default**: `1000`

## Deduplication Configuration

### DEDUP_ENABLED

**Optional**

Enable code execution deduplication.

**Default**: `false`

### DEDUP_CACHE_TTL

**Optional**

Deduplication cache TTL in seconds.

**Default**: `3600` (1 hour)

### DEDUP_CACHE_TYPE

**Optional**

Deduplication cache type: `memory` | `redis`

**Default**: `memory`

### REDIS_URL

**Required when DEDUP_CACHE_TYPE=redis**

Redis connection URL.

**Example**: `redis://localhost:6379`

## Server Configuration

### PORT

**Optional**

HTTP server port.

**Default**: `8000`

### GRPC_PORT

**Optional**

gRPC server port.

**Default**: `50051`

### RUST_LOG

**Optional**

Log level: `error` | `warn` | `info` | `debug` | `trace`

**Default**: `info`

## Provider-Specific Configurations

### Firebase Authentication

```bash
AUTH_TYPE=jwt
JWT_ISSUER_URL=https://securetoken.google.com/your-project-id
JWT_AUDIENCE=your-project-id
JWT_PUBLIC_KEY_URL=https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com
```

### Microsoft Azure AD

```bash
AUTH_TYPE=jwt
JWT_ISSUER_URL=https://login.microsoftonline.com/your-tenant-id/v2.0
JWT_AUDIENCE=your-app-id
JWT_PUBLIC_KEY_URL=https://login.microsoftonline.com/your-tenant-id/discovery/v2.0/keys
```

### Auth0

```bash
AUTH_TYPE=jwt
JWT_ISSUER_URL=https://your-domain.auth0.com/
JWT_AUDIENCE=your-api-identifier
JWT_PUBLIC_KEY_URL=https://your-domain.auth0.com/.well-known/jwks.json
```

### Google OAuth 2.0

```bash
AUTH_TYPE=oauth2
OAUTH2_PROVIDER=google
OAUTH2_CLIENT_ID=your-client-id
OAUTH2_CLIENT_SECRET=your-client-secret
OAUTH2_TOKEN_URL=https://oauth2.googleapis.com/token
OAUTH2_USERINFO_URL=https://www.googleapis.com/oauth2/v2/userinfo
```

### GitHub OAuth 2.0

```bash
AUTH_TYPE=oauth2
OAUTH2_PROVIDER=github
OAUTH2_CLIENT_ID=your-client-id
OAUTH2_CLIENT_SECRET=your-client-secret
OAUTH2_TOKEN_URL=https://github.com/login/oauth/access_token
OAUTH2_USERINFO_URL=https://api.github.com/user
```

## Docker Configuration Examples

### Development Environment

```bash
docker run -p 8000:8000 -p 50051:50051 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=none \
  -e CORS_ENABLED=true \
  -e CORS_ALLOWED_ORIGINS=* \
  --user root \
  isobox:latest
```

### Production with API Keys

```bash
docker run -p 8000:8000 -p 50051:50051 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS=prod-key-1,prod-key-2,prod-key-3 \
  -e CORS_ENABLED=true \
  -e CORS_ALLOWED_ORIGINS=https://yourdomain.com \
  -e CORS_ALLOW_CREDENTIALS=true \
  --user root \
  isobox:latest
```

### Production with JWT (Firebase)

```bash
docker run -p 8000:8000 -p 50051:50051 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=jwt \
  -e JWT_ISSUER_URL=https://securetoken.google.com/your-project-id \
  -e JWT_AUDIENCE=your-project-id \
  -e JWT_PUBLIC_KEY_URL=https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com \
  -e CORS_ENABLED=true \
  -e CORS_ALLOWED_ORIGINS=https://yourdomain.com \
  --user root \
  isobox:latest
```

### Production with Redis Cache

```bash
docker run -p 8000:8000 -p 50051:50051 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS=prod-key-1,prod-key-2 \
  -e DEDUP_ENABLED=true \
  -e DEDUP_CACHE_TYPE=redis \
  -e REDIS_URL=redis://redis:6379 \
  -e CORS_ENABLED=true \
  -e CORS_ALLOWED_ORIGINS=https://yourdomain.com \
  --user root \
  isobox:latest
```

## Environment Variable Reference

| Variable                    | Required | Default                                | Description              |
| --------------------------- | -------- | -------------------------------------- | ------------------------ |
| `AUTH_TYPE`                 | No       | `none`                                 | Authentication type      |
| `JWT_ISSUER_URL`            | JWT      | -                                      | JWT issuer URL           |
| `JWT_AUDIENCE`              | JWT      | -                                      | JWT audience             |
| `JWT_PUBLIC_KEY_URL`        | JWT      | -                                      | JWT public key URL       |
| `JWT_CACHE_TTL`             | No       | `3600`                                 | JWT cache TTL            |
| `API_KEYS`                  | API Key  | -                                      | Comma-separated API keys |
| `API_KEY_HEADER_NAME`       | No       | `X-API-Key`                            | API key header name      |
| `MTLS_CA_CERT_PATH`         | mTLS     | -                                      | CA certificate path      |
| `MTLS_CLIENT_CERT_REQUIRED` | No       | `true`                                 | Require client certs     |
| `MTLS_VERIFY_HOSTNAME`      | No       | `true`                                 | Verify hostname          |
| `OAUTH2_PROVIDER`           | OAuth2   | -                                      | OAuth2 provider          |
| `OAUTH2_CLIENT_ID`          | OAuth2   | -                                      | OAuth2 client ID         |
| `OAUTH2_CLIENT_SECRET`      | OAuth2   | -                                      | OAuth2 client secret     |
| `OAUTH2_TOKEN_URL`          | OAuth2   | -                                      | OAuth2 token URL         |
| `OAUTH2_USERINFO_URL`       | OAuth2   | -                                      | OAuth2 userinfo URL      |
| `CORS_ENABLED`              | No       | `false`                                | Enable CORS              |
| `CORS_ALLOWED_ORIGINS`      | CORS     | -                                      | Allowed origins          |
| `CORS_ALLOWED_METHODS`      | No       | `GET,POST,PUT,DELETE,OPTIONS`          | Allowed methods          |
| `CORS_ALLOWED_HEADERS`      | No       | `Content-Type,Authorization,X-API-Key` | Allowed headers          |
| `CORS_ALLOW_CREDENTIALS`    | No       | `false`                                | Allow credentials        |
| `CORS_MAX_AGE`              | No       | -                                      | CORS max age             |
| `AUTH_CACHE_TTL`            | No       | `3600`                                 | Auth cache TTL           |
| `AUTH_CACHE_MAX_SIZE`       | No       | `1000`                                 | Auth cache max size      |
| `DEDUP_ENABLED`             | No       | `false`                                | Enable deduplication     |
| `DEDUP_CACHE_TTL`           | No       | `3600`                                 | Dedup cache TTL          |
| `DEDUP_CACHE_TYPE`          | No       | `memory`                               | Dedup cache type         |
| `REDIS_URL`                 | Redis    | -                                      | Redis URL                |
| `PORT`                      | No       | `8000`                                 | HTTP port                |
| `GRPC_PORT`                 | No       | `50051`                                | gRPC port                |
| `RUST_LOG`                  | No       | `info`                                 | Log level                |

## Security Considerations

### Production Recommendations

1. **Never use `AUTH_TYPE=none` in production**
2. **Use strong, unique API keys**
3. **Configure CORS with specific origins, not `*`**
4. **Use HTTPS in production**
5. **Set appropriate cache TTLs**
6. **Monitor authentication logs**
7. **Use Redis for caching in multi-instance deployments**

### Environment Variable Security

1. **Store sensitive values in environment variables, not in code**
2. **Use secrets management for production deployments**
3. **Rotate API keys and JWT secrets regularly**
4. **Limit CORS origins to specific domains**
5. **Use appropriate log levels in production**

## Troubleshooting

### Common Issues

1. **Authentication fails**: Check environment variables and provider configuration
2. **CORS errors**: Verify `CORS_ALLOWED_ORIGINS` includes your domain
3. **Cache issues**: Check Redis connection if using Redis cache
4. **Performance issues**: Adjust cache TTLs and enable deduplication

### Debug Mode

Enable debug logging to troubleshoot issues:

```bash
RUST_LOG=debug cargo run
```

This will show detailed authentication and configuration information.
