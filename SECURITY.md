# Security

This document outlines the security features, best practices, and policies for IsoBox.

## Table of Contents

1. [Security Features](#security-features)
2. [Security Architecture](#security-architecture)
3. [Authentication & Authorization](#authentication--authorization)
4. [Container Security](#container-security)
5. [Network Security](#network-security)
6. [Data Security](#data-security)
7. [Vulnerability Reporting](#vulnerability-reporting)
8. [Security Best Practices](#security-best-practices)
9. [Security Checklist](#security-checklist)
10. [Incident Response](#incident-response)

## Security Features

### Core Security Features

- **Container Isolation**: Each code execution runs in a separate Docker container
- **Resource Limits**: CPU, memory, process, and file descriptor limits
- **Network Isolation**: Containers run with `--network none`
- **Privilege Dropping**: Containers run with dropped capabilities
- **Multi-Authentication**: Support for API keys, JWT, OAuth2, and mTLS
- **Code Deduplication**: Hash-based caching to prevent duplicate execution
- **Timeout Protection**: Configurable execution timeouts
- **Input Validation**: Comprehensive request validation

### Security Layers

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Input     │  │   Request   │  │   Response          │  │
│  │ Validation  │  │ Validation  │  │ Sanitization        │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Authentication Layer                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   API Key   │  │     JWT     │  │      OAuth2         │  │
│  │     Auth    │  │     Auth    │  │       Auth          │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Execution Layer                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Resource  │  │   Container │  │   Code              │  │
│  │   Limits    │  │  Isolation  │  │  Deduplication      │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Container Layer                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Network   │  │   Privilege │  │   File System       │  │
│  │  Isolation  │  │   Dropping  │  │   Isolation         │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Security Architecture

### System Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client        │    │   Load Balancer │    │   Firewall      │
│   (HTTPS)       │───▶│   (SSL/TLS)     │───▶│   (WAF)         │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                                        │
                                                        ▼
┌─────────────────────────────────────────────────────────────────┐
│                        IsoBox Server                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   Rate      │  │   Input     │  │   Authentication        │  │
│  │  Limiting   │  │ Validation  │  │   Middleware            │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │                    Code Executor                           │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │  │
│  │  │   Resource  │  │   Container │  │   Security          │  │  │
│  │  │   Limits    │  │   Manager   │  │   Monitor           │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────────────┘  │  │
│  └─────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Docker Runtime                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   Security  │  │   Resource  │  │   Network               │  │
│  │   Profiles  │  │   Control   │  │   Policies              │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### Security Components

1. **Input Validation Layer**

   - Request size limits
   - Content type validation
   - Code syntax validation
   - Malicious pattern detection

2. **Authentication Layer**

   - Multi-factor authentication support
   - Token validation
   - Session management
   - Rate limiting

3. **Execution Layer**

   - Resource monitoring
   - Container lifecycle management
   - Security event logging
   - Performance monitoring

4. **Container Layer**
   - Security profiles
   - Resource isolation
   - Network policies
   - File system restrictions

## Authentication & Authorization

### Authentication Methods

#### 1. API Key Authentication

```bash
# Secure API key configuration
AUTH_TYPE=apikey
API_KEYS=key1,key2,key3
API_KEY_HEADER=X-API-Key

# Usage
curl -X POST http://localhost:8000/api/v1/execute \
  -H "X-API-Key: your-secure-api-key" \
  -H "Content-Type: application/json" \
  -d '{"language": "python", "code": "print(\"Hello\")"}'
```

**Security Considerations:**

- Use strong, randomly generated API keys
- Rotate keys regularly
- Store keys securely (environment variables, secrets)
- Use HTTPS in production
- Implement rate limiting

#### 2. JWT Authentication

```bash
# JWT configuration
AUTH_TYPE=jwt
JWT_ISSUER_URL=https://accounts.google.com
JWT_AUDIENCE=your-app-id
JWT_PUBLIC_KEY_URL=https://www.googleapis.com/oauth2/v1/certs

# Usage
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"language": "python", "code": "print(\"Hello\")"}'
```

**Security Considerations:**

- Validate JWT signature
- Check token expiration
- Verify issuer and audience
- Use secure key storage
- Implement token refresh

#### 3. OAuth2 Authentication

```bash
# OAuth2 configuration
AUTH_TYPE=oauth2
OAUTH2_PROVIDER=firebase
OAUTH2_CLIENT_ID=your-client-id
OAUTH2_CLIENT_SECRET=your-client-secret
OAUTH2_TOKEN_URL=https://oauth2.googleapis.com/token
OAUTH2_USERINFO_URL=https://www.googleapis.com/oauth2/v2/userinfo

# Usage
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Authorization: Bearer YOUR_OAUTH2_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"language": "python", "code": "print(\"Hello\")"}'
```

**Security Considerations:**

- Validate OAuth2 tokens with provider
- Check token scope and permissions
- Implement proper error handling
- Use secure token storage
- Monitor authentication events

#### 4. mTLS Authentication

```bash
# mTLS configuration
AUTH_TYPE=mtls
MTLS_CA_CERT=/path/to/ca.crt
MTLS_CLIENT_CERT=/path/to/client.crt
MTLS_CLIENT_KEY=/path/to/client.key

# Usage with client certificate
curl -X POST https://localhost:8000/api/v1/execute \
  --cert client.crt \
  --key client.key \
  --cacert ca.crt \
  -H "Content-Type: application/json" \
  -d '{"language": "python", "code": "print(\"Hello\")"}'
```

**Security Considerations:**

- Use strong certificate authorities
- Implement certificate revocation
- Monitor certificate expiration
- Use secure key storage
- Implement proper error handling

### Authorization

#### Permission-Based Access Control

```rust
// Example permission structure
pub struct Permission {
    pub resource: String,
    pub action: String,
    pub conditions: Vec<Condition>,
}

pub struct Condition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

// Example permissions
let permissions = vec![
    Permission {
        resource: "code_execution".to_string(),
        action: "execute".to_string(),
        conditions: vec![
            Condition {
                field: "language".to_string(),
                operator: "in".to_string(),
                value: "python,nodejs,java".to_string(),
            },
            Condition {
                field: "max_execution_time".to_string(),
                operator: "lte".to_string(),
                value: "30".to_string(),
            },
        ],
    },
];
```

## Container Security

### Container Isolation

#### 1. Resource Limits

```bash
# Docker run with resource limits
docker run \
  --memory=512m \
  --cpus=1.0 \
  --pids-limit=100 \
  --ulimit nofile=1024:1024 \
  --security-opt no-new-privileges \
  --cap-drop=ALL \
  --network=none \
  isobox/isobox:latest
```

#### 2. Security Profiles

```bash
# AppArmor profile
#include <tunables/global>

profile isobox flags=(attach_disconnected,mediate_deleted) {
  #include <abstractions/base>
  #include <abstractions/docker>

  # Deny dangerous operations
  deny /proc/sys/kernel/core_pattern w,
  deny /proc/sys/kernel/modprobe w,
  deny /proc/sys/kernel/hotplug w,

  # Allow necessary operations
  /tmp/** rw,
  /var/run/docker.sock rw,
}
```

#### 3. Seccomp Profile

```json
{
  "defaultAction": "SCMP_ACT_ERRNO",
  "architectures": ["SCMP_ARCH_X86_64"],
  "syscalls": [
    {
      "names": ["read", "write", "exit", "exit_group"],
      "action": "SCMP_ACT_ALLOW"
    },
    {
      "names": ["execve", "clone", "fork"],
      "action": "SCMP_ACT_ALLOW",
      "args": [],
      "comment": "Allow basic process operations"
    }
  ]
}
```

### Container Hardening

#### 1. Non-Root User

```dockerfile
# Create non-root user
RUN useradd -m -u 1000 -s /bin/bash isobox
USER isobox

# Set proper permissions
RUN chown -R isobox:isobox /app
```

#### 2. Read-Only Root Filesystem

```bash
# Run with read-only root filesystem
docker run \
  --read-only \
  --tmpfs /tmp \
  --tmpfs /var/run \
  isobox/isobox:latest
```

#### 3. Security Scanning

```bash
# Scan for vulnerabilities
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  aquasec/trivy image isobox/isobox:latest

# Scan for secrets
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  aquasec/trivy fs --security-checks secret .
```

## Network Security

### Network Isolation

#### 1. Container Network Policies

```bash
# Run container with no network access
docker run --network=none isobox/isobox:latest

# Use custom network with restrictions
docker network create --driver bridge --subnet=172.20.0.0/16 isobox-net
docker run --network=isobox-net isobox/isobox:latest
```

#### 2. Firewall Rules

```bash
# iptables rules for IsoBox
iptables -A INPUT -p tcp --dport 8000 -s 192.168.1.0/24 -j ACCEPT
iptables -A INPUT -p tcp --dport 9000 -s 192.168.1.0/24 -j ACCEPT
iptables -A INPUT -p tcp --dport 8000 -j DROP
iptables -A INPUT -p tcp --dport 9000 -j DROP
```

#### 3. TLS/SSL Configuration

```rust
// TLS configuration example
use rustls::{ServerConfig, PrivateKey, Certificate};
use std::fs::File;
use std::io::BufReader;

let mut config = ServerConfig::new(NoClientAuth::new());
let cert_file = &mut BufReader::new(File::open("cert.pem")?);
let key_file = &mut BufReader::new(File::open("key.pem")?);
let cert_chain = certs(cert_file)?;
let mut keys: Vec<PrivateKey> = keys(key_file)?;
config.set_single_cert(cert_chain, keys.remove(0))?;
```

## Data Security

### Data Protection

#### 1. Encryption at Rest

```bash
# Encrypt sensitive data
echo "sensitive-data" | openssl enc -aes-256-cbc -salt -out encrypted.txt

# Use encrypted volumes
docker run -v encrypted-volume:/data isobox/isobox:latest
```

#### 2. Encryption in Transit

```bash
# Use HTTPS/TLS
curl -X POST https://localhost:8000/api/v1/execute \
  --cert client.crt \
  --key client.key \
  --cacert ca.crt \
  -H "Content-Type: application/json" \
  -d '{"language": "python", "code": "print(\"Hello\")"}'
```

#### 3. Data Sanitization

```rust
// Input sanitization example
pub fn sanitize_code(code: &str) -> String {
    code.chars()
        .filter(|&c| c.is_ascii() && !c.is_control())
        .collect()
}

// Output sanitization
pub fn sanitize_output(output: &str) -> String {
    output.chars()
        .filter(|&c| c.is_ascii() && !c.is_control())
        .collect()
}
```

### Data Retention

#### 1. Log Retention

```bash
# Configure log rotation
logrotate /etc/logrotate.d/isobox

# Example logrotate configuration
/var/log/isobox/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 isobox isobox
}
```

#### 2. Cache Management

```rust
// Cache TTL configuration
pub struct CacheConfig {
    pub ttl: Duration,
    pub max_size: usize,
    pub cleanup_interval: Duration,
}

// Automatic cache cleanup
pub async fn cleanup_cache(cache: &Cache) {
    let mut interval = tokio::time::interval(Duration::from_secs(3600));
    loop {
        interval.tick().await;
        cache.cleanup().await;
    }
}
```

## Vulnerability Reporting

### Reporting Security Issues

We take security seriously. If you discover a security vulnerability, please follow these steps:

1. **DO NOT** create a public GitHub issue
2. **DO** email us at security@isobox.dev
3. **DO** include detailed information about the vulnerability
4. **DO** provide steps to reproduce the issue
5. **DO** suggest potential fixes if possible

### Security Contact Information

- **Email**: security@isobox.dev
- **PGP Key**: [Security PGP Key](https://isobox.dev/security.asc)
- **Response Time**: Within 48 hours
- **Disclosure Policy**: Coordinated disclosure

### Vulnerability Response Process

1. **Acknowledgment**: We will acknowledge receipt within 48 hours
2. **Investigation**: Our security team will investigate the issue
3. **Fix Development**: We will develop and test a fix
4. **Release**: We will release a security update
5. **Disclosure**: We will publicly disclose the vulnerability

### Security Advisories

Security advisories are published at:

- [GitHub Security Advisories](https://github.com/isobox/isobox/security/advisories)
- [Security Blog](https://isobox.dev/security)
- [Security Mailing List](https://groups.google.com/g/isobox-security)

## Security Best Practices

### Deployment Security

#### 1. Production Checklist

- [ ] Use HTTPS/TLS in production
- [ ] Configure proper authentication
- [ ] Set up monitoring and alerting
- [ ] Implement rate limiting
- [ ] Use secure secrets management
- [ ] Configure firewall rules
- [ ] Enable security scanning
- [ ] Set up log monitoring
- [ ] Configure backup and recovery
- [ ] Test security measures

#### 2. Environment Security

```bash
# Secure environment configuration
export AUTH_TYPE=apikey
export API_KEYS=$(openssl rand -hex 32)
export API_KEY_HEADER=X-API-Key
export REST_PORT=8000
export GRPC_PORT=9000
export DEDUP_ENABLED=true
export DEDUP_CACHE_TTL=3600
export RUST_LOG=info
```

#### 3. Container Security

```bash
# Security-focused Docker run
docker run -d \
  --name isobox \
  --security-opt no-new-privileges \
  --cap-drop=ALL \
  --read-only \
  --tmpfs /tmp \
  --tmpfs /var/run \
  --network=none \
  --memory=1g \
  --cpus=1.0 \
  --pids-limit=100 \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="$API_KEYS" \
  isobox/isobox:latest
```

### Code Security

#### 1. Input Validation

```rust
// Comprehensive input validation
pub fn validate_execution_request(request: &ExecutionRequest) -> Result<(), ValidationError> {
    // Validate language
    if !is_supported_language(&request.language) {
        return Err(ValidationError::UnsupportedLanguage(request.language.clone()));
    }

    // Validate code size
    if request.code.len() > MAX_CODE_SIZE {
        return Err(ValidationError::CodeTooLarge(request.code.len()));
    }

    // Validate code content
    if contains_malicious_patterns(&request.code) {
        return Err(ValidationError::MaliciousCode);
    }

    Ok(())
}
```

#### 2. Error Handling

```rust
// Secure error handling
pub fn handle_error(error: &Error) -> HttpResponse {
    match error {
        Error::AuthenticationFailed => {
            HttpResponse::Unauthorized().json(json!({
                "error": "Authentication failed",
                "message": "Invalid credentials"
            }))
        },
        Error::ValidationFailed(msg) => {
            HttpResponse::BadRequest().json(json!({
                "error": "Validation failed",
                "message": msg
            }))
        },
        _ => {
            // Don't leak internal errors
            HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error",
                "message": "An unexpected error occurred"
            }))
        }
    }
}
```

#### 3. Logging Security

```rust
// Secure logging
use log::{info, warn, error};

// Don't log sensitive data
info!("Code execution request received for language: {}", language);
warn!("Rate limit exceeded for user: {}", user_id);
error!("Authentication failed for user: {}", user_id);

// Sanitize logs
pub fn sanitize_log_message(message: &str) -> String {
    // Remove sensitive patterns
    message.replace("password=", "password=***")
           .replace("token=", "token=***")
           .replace("key=", "key=***")
}
```

## Security Checklist

### Pre-Deployment Checklist

- [ ] **Authentication**: Configure secure authentication
- [ ] **Authorization**: Set up proper access controls
- [ ] **Network**: Configure firewall and network policies
- [ ] **TLS**: Enable HTTPS/TLS encryption
- [ ] **Secrets**: Use secure secrets management
- [ ] **Monitoring**: Set up security monitoring
- [ ] **Logging**: Configure secure logging
- [ ] **Backup**: Set up secure backup procedures
- [ ] **Testing**: Perform security testing
- [ ] **Documentation**: Update security documentation

### Runtime Security Checklist

- [ ] **Container Isolation**: Verify container isolation
- [ ] **Resource Limits**: Check resource limits
- [ ] **Network Security**: Monitor network access
- [ ] **Authentication**: Verify authentication is working
- [ ] **Authorization**: Check authorization policies
- [ ] **Logging**: Monitor security logs
- [ ] **Performance**: Monitor for performance issues
- [ ] **Updates**: Keep system updated
- [ ] **Backup**: Verify backup procedures
- [ ] **Incident Response**: Test incident response procedures

### Security Monitoring

#### 1. Log Monitoring

```bash
# Monitor authentication logs
tail -f /var/log/isobox/auth.log | grep -i "failed\|error\|warning"

# Monitor execution logs
tail -f /var/log/isobox/execution.log | grep -i "timeout\|memory\|error"

# Monitor system logs
journalctl -u isobox -f | grep -i "security\|error\|warning"
```

#### 2. Performance Monitoring

```bash
# Monitor resource usage
docker stats isobox

# Monitor network connections
netstat -tulpn | grep :8000
netstat -tulpn | grep :9000

# Monitor disk usage
df -h /tmp
du -sh /var/log/isobox
```

#### 3. Security Alerts

```bash
# Set up security alerts
# Example: Alert on failed authentication attempts
grep "Authentication failed" /var/log/isobox/auth.log | wc -l

# Example: Alert on resource usage
docker stats --no-stream isobox | awk 'NR>1 {if($3 > 80) print "High CPU usage"}'
```

## Incident Response

### Incident Response Plan

#### 1. Detection

- Monitor logs for suspicious activity
- Set up automated alerts
- Use security monitoring tools
- Regular security assessments

#### 2. Analysis

- Investigate the incident
- Determine scope and impact
- Identify root cause
- Document findings

#### 3. Containment

- Isolate affected systems
- Block malicious traffic
- Revoke compromised credentials
- Implement temporary fixes

#### 4. Eradication

- Remove malicious code
- Patch vulnerabilities
- Update security measures
- Verify system integrity

#### 5. Recovery

- Restore from backups
- Verify system functionality
- Monitor for recurrence
- Update incident response procedures

#### 6. Lessons Learned

- Document lessons learned
- Update security procedures
- Improve monitoring
- Train staff

### Incident Response Contacts

- **Security Team**: security@isobox.dev
- **Operations Team**: ops@isobox.dev
- **Management**: management@isobox.dev
- **Legal**: legal@isobox.dev

### Incident Response Procedures

#### 1. Security Breach

1. **Immediate Actions**

   - Isolate affected systems
   - Preserve evidence
   - Notify security team
   - Document incident

2. **Investigation**

   - Analyze logs and evidence
   - Determine attack vector
   - Assess impact
   - Identify affected users

3. **Remediation**

   - Patch vulnerabilities
   - Remove malicious code
   - Update security measures
   - Verify system integrity

4. **Communication**
   - Notify affected users
   - Issue security advisory
   - Update stakeholders
   - Provide guidance

#### 2. Data Breach

1. **Immediate Actions**

   - Secure affected systems
   - Preserve evidence
   - Notify legal team
   - Document incident

2. **Assessment**

   - Determine data types affected
   - Assess regulatory requirements
   - Identify affected individuals
   - Calculate potential impact

3. **Notification**

   - Notify regulatory authorities
   - Notify affected individuals
   - Issue public statement
   - Provide guidance

4. **Remediation**
   - Implement additional security
   - Update procedures
   - Provide credit monitoring
   - Review policies

---

For more information, see the [main README](README.md) and other documentation files.
