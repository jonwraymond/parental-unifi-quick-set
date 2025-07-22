# Parental Unifi Quick Set

A simple web-based tool for quickly setting up parental controls on UniFi networks. Block apps like Fortnite, Roblox, or YouTube with flexible scheduling options.

## Features

- **Easy Login**: Connect to your UniFi controller with your credentials
- **App Blocking**: Block popular apps and games by name
- **Flexible Scheduling**:
  - Permanent blocks
  - Temporary blocks (for X hours)
  - Block until a specific time
  - Recurring schedules (e.g., block gaming apps on school nights)
- **Device Selection**: Choose which devices to apply rules to
- **Network Selection**: Apply rules to specific VLANs or networks

## Prerequisites

- UniFi Dream Machine or UniFi Controller
- Rust (for building from source)
- Docker (for containerized deployment)

## Configuration

Before running, update the UniFi controller URL in `src/main.rs`:

```rust
unifi_url: "https://192.168.1.1:8443".to_string(),  // Update to your UniFi controller
```

## Building

### Local Development

```bash
cargo build --release
cargo run
```

The application will be available at `http://localhost:3000`

### Docker

```bash
docker build -t parental-unifi-quick-set .
docker run -p 3000:3000 parental-unifi-quick-set
```

## Deployment Options

### 1. Docker Compose (Recommended)

```bash
docker-compose up -d
```

### 2. Fly.io

```bash
fly launch
fly deploy
```

### 3. Systemd Service (Linux)

Copy the binary and create a systemd service file.

## Usage

1. Navigate to `http://localhost:3000` (or your deployed URL)
2. Login with your UniFi credentials
3. Select apps to block
4. Choose blocking schedule
5. Select target networks and devices
6. Create the rule

## Security Notes

- The app accepts self-signed certificates from UniFi controllers
- Credentials are not stored - authentication tokens are kept in memory only
- Consider using HTTPS in production

## License

MIT
