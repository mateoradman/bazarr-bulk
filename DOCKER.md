# Docker Guide for bazarr-bulk

This guide explains how to run bazarr-bulk using Docker and Docker Compose.

## Quick Start

### Prerequisites

- Docker installed on your system
- Docker Compose (optional, for easier management)
- A `config.json` file with your Bazarr credentials

### 1. Create Configuration File

Create a `config.json` file in your project directory:

```json
{
  "host": "your-bazarr-host",
  "protocol": "http",
  "apiKey": "your-api-key",
  "port": "6767",
  "baseUrl": ""
}
```

### 2. Using Docker Run

#### Build the image locally:

```bash
docker build -t bazarr-bulk .
```

#### Run with Docker:

```bash
# Example: Sync movies
docker run --rm \
  -v "$(pwd)/config.json:/config/config.json:ro" \
  -v bazarr-bulk-data:/data \
  bazarr-bulk --config /config/config.json movies sync

# Example: Process TV shows with OCR fixes
docker run --rm \
  -v "$(pwd)/config.json:/config/config.json:ro" \
  -v bazarr-bulk-data:/data \
  bazarr-bulk --config /config/config.json tv-shows ocr-fixes

# Example: With skip-processed flag
docker run --rm \
  -v "$(pwd)/config.json:/config/config.json:ro" \
  -v bazarr-bulk-data:/data \
  bazarr-bulk --config /config/config.json movies sync --skip-processed
```

### 3. Using Docker Compose

Docker Compose makes it easier to manage volumes and configurations.

#### Edit docker-compose.yml

Modify the `command` section to run your desired action:

```yaml
command: ["--config", "/config/config.json", "movies", "sync"]
```

#### Run with Docker Compose:

```bash
# Run once and remove
docker compose run --rm bazarr-bulk

# Or build and run
docker compose up --build

# Run with custom command
docker compose run --rm bazarr-bulk --config /config/config.json tv-shows ocr-fixes
```

### 4. Using Pre-built Image from GitHub Container Registry

Pull the latest image:

```bash
docker pull ghcr.io/mateoradman/bazarr-bulk:latest
```

Run with the pre-built image:

```bash
docker run --rm \
  -v "$(pwd)/config.json:/config/config.json:ro" \
  -v bazarr-bulk-data:/data \
  ghcr.io/mateoradman/bazarr-bulk:latest \
  --config /config/config.json movies sync
```

Or with `docker-compose.yml`:

```yaml
services:
  bazarr-bulk:
    image: ghcr.io/mateoradman/bazarr-bulk:latest
    # ... rest of config
```

## Available Actions

All CLI options are available in the Docker container:

```bash
# Show help
docker run --rm bazarr-bulk --help

# Movies actions
docker run --rm -v "$(pwd)/config.json:/config/config.json:ro" -v bazarr-bulk-data:/data \
  bazarr-bulk --config /config/config.json movies [ACTION]

# TV Shows actions
docker run --rm -v "$(pwd)/config.json:/config/config.json:ro" -v bazarr-bulk-data:/data \
  bazarr-bulk --config /config/config.json tv-shows [ACTION]
```

Available actions:
- `sync`
- `ocr-fixes`
- `common-fixes`
- `remove-hearing-impaired`
- `remove-style-tags`
- `reverse-rtl`

### Persistent Database

The SQLite database is stored in the `/data` volume by default to track processed subtitles. This allows you to use the `--skip-processed` flag across container restarts.

#### Database Location Options

The database location can be configured in three ways (in order of priority):

1. **CLI argument** - Most specific

```bash
docker run --rm \
  -v "$(pwd)/config.json:/config/config.json:ro" \
  -v "$(pwd)/data:/data" \
  bazarr-bulk --config /config/config.json --db-path /data/custom.db movies sync
```

2. **Environment variable**

```bash
docker run --rm \
  -v "$(pwd)/config.json:/config/config.json:ro" \
  -v bazarr-bulk-data:/data \
  -e BB_DATA_DIR=/data \
  bazarr-bulk --config /config/config.json movies sync
```

3. **Default location** - Falls back to system data directory

**Recommendation:** For Docker, use the `BB_DATA_DIR` environment variable (already configured in `docker-compose.yml`) and mount a volume to `/data`.

#### Managing the Database

To inspect or backup the database:

```bash
# List volume
docker volume ls | grep bazarr-bulk

# Inspect volume location
docker volume inspect bazarr-bulk-data

# Backup database
docker run --rm \
  -v bazarr-bulk-data:/data \
  -v "$(pwd):/backup" \
  alpine tar czf /backup/bazarr-bulk-backup.tar.gz /data

# Restore database
docker run --rm \
  -v bazarr-bulk-data:/data \
  -v "$(pwd):/backup" \
  alpine tar xzf /backup/bazarr-bulk-backup.tar.gz -C /

# View database location on startup
# The application will print: "Using database at: /data/database.db"
```

### Scheduling with Cron

You can schedule bazarr-bulk to run periodically using cron:

```bash
# Edit crontab
crontab -e

# Add entry to run daily at 2 AM
0 2 * * * docker run --rm -v /path/to/config.json:/config/config.json:ro -v bazarr-bulk-data:/data ghcr.io/mateoradman/bazarr-bulk:latest --config /config/config.json movies sync --skip-processed
```

## Troubleshooting

### Cannot connect to Bazarr

- Ensure the `host` in your config.json is accessible from inside the container
- Use `host.docker.internal` for local Bazarr on Mac/Windows
- Use your machine's IP address for local Bazarr on Linux
- Use container name if Bazarr is in Docker on the same network

### View logs

```bash
# With docker run (outputs to console)
docker run --rm -v "$(pwd)/config.json:/config/config.json:ro" -v bazarr-bulk-data:/data \
  bazarr-bulk --config /config/config.json movies sync

# With docker compose - follow logs
docker compose logs -f bazarr-bulk

# View logs with timestamps
docker compose logs -f -t bazarr-bulk
```

**Note:** When running in Docker or systemd, you'll see plain text progress updates instead of animated progress bars, making it easier to track progress in log files.

## Image Variants

- `latest` - Latest stable release
- `vX.X.X` - Specific version tags
- `main` - Latest build from main branch (may be unstable)
