# BAZARR BULK CLI

A CLI tool for performing actions in bulk on Bazarr movies and TV shows. The tool maintains a local database to track processed subtitles, allowing you to skip previously processed items using the `--skip-processed` flag.

## Database Storage

The SQLite database location can be configured in three ways (in order of priority):

1. **CLI argument**: `--db-path /path/to/database.db`
2. **Environment variable**: `BB_DATA_DIR=/path/to/directory` (database will be created as `database.db` in this directory)
3. **Default locations** (when none specified):
   - Linux: `~/.local/share/bazarr-bulk/database.db`
   - Windows: `C:\Users\<username>\AppData\Local\mateoradman\bazarr-bulk\database.db`
   - macOS: `~/Library/Application Support/com.mateoradman.bazarr-bulk/database.db`

For Docker usage, the database is automatically stored in `/data` volume (see [DOCKER.md](./DOCKER.md)).

List of supported actions:

- sync
- ocr-fixes
- common-fixes
- remove-hearing-impaired
- remove-style-tags
- reverse-rtl

## Installation

### Docker (Recommended)

The easiest way to run bazarr-bulk is using Docker:

```sh
# Pull the latest image
docker pull ghcr.io/mateoradman/bazarr-bulk:latest

# Run with your config
docker run --rm \
  -v "$(pwd)/config.json:/config/config.json:ro" \
  -v bazarr-bulk-data:/data \
  ghcr.io/mateoradman/bazarr-bulk:latest \
  --config /config/config.json movies sync
```

Or use Docker Compose for easier management. See [DOCKER.md](./DOCKER.md) for detailed Docker usage instructions.

### Install with cargo

[bazarr-bulk](https://crates.io/crates/bazarr-bulk) is published on crates.io.  
In order to install a Rust crate from crates.io, it is required to have [Rust and cargo installed](https://doc.rust-lang.org/cargo/getting-started/installation.html) on your system.

```sh
cargo install bazarr-bulk
```

### Manual installation from an archive

[Latest release](https://github.com/mateoradman/bazarr-bulk/releases/latest) page provides an option to manually install the bb binary from an archive. The archive is available for Linux, MacOS, and Windows.  
Download, extract and move the binary to the desired directory, and set execution permissions.

#### Linux

1. Download the Linux tar.gz archive from the latest [release](https://github.com/mateoradman/bazarr-bulk/releases/latest)
2. Extract the archive

```sh
tar xf bazarr-bulk-linux-x86_64.tar.gz
```

3. Move the binary

```sh
sudo mv bb /usr/local/bin
```

4. Set execution permissions

```sh
sudo chmod +x /usr/local/bin/bb
```

5. Run bb

```sh
bb --help
```

#### MacOS

1. Download the MacOS (apple-darwin) ZIP archive from the latest [release](https://github.com/mateoradman/bazarr-bulk/releases/latest)
2. Extract the archive

```sh
unzip bazarr-bulk-macos-x86_64.zip
```

3. Move the binary

```sh
sudo mv bb /usr/local/bin
```

4. Set execution permissions

```sh
sudo chmod +x /usr/local/bin/bb
```

5. Run bb

```sh
bb --help
```

#### Windows

1. Download the Windows ZIP archive from the latest [release](https://github.com/mateoradman/bazarr-bulk/releases/latest)
2. Extract the archive
3. Run bb.exe

## Configuration File

The [configuration file](./examples/config.json) contains various fields to set up and communicate with Bazarr. Below is a breakdown of each field and its purpose:

- **`host`**:  
  Defines Bazarr's IP address.

  - Default: `"0.0.0.0"`

- **`port`** (optional):  
  Defines Bazarr's port. Can be omitted if Bazarr is accessible through ports 80 (HTTP) or 443 (HTTPS).

  - Default: `"6767"` (You can change this to any available port on your server).

- **`protocol`**:  
  Specifies the protocol to be used by the service (HTTP or HTTPS). **Note: Bazarr must be available using the specified protocol.**

  - Default: `"http"`.

- **`apiKey`**:  
  The [API key](https://wiki.bazarr.media/Additional-Configuration/Webhooks/#where-can-i-find-the-bazarr-api-key) used to authenticate Bazarr requests.

  - Replace `<YOUR_API_KEY>` with the actual Bazarr API key.

- **`baseUrl`** (optional):  
  The [base URL](https://wiki.bazarr.media/Additional-Configuration/Settings/#url-base) or path at which the service will be accessible.

  - Default: `""` (You can modify this based on your application's routing needs).

  Example:
  - `http://MY-IP:MY-PORT/` baseUrl should be an empty string or omitted.
  - `http://MY-IP:MY-PORT/bazarr/` - baseUrl should be `"bazarr"`.

## Usage

1. Create a JSON config file based on the template [file](./examples/config.json).
2. Run `bb --config your-config.json` [movies|tv-shows] [ACTION]

### CLI Manual

```bash
bb --help
# Performs bulk operations on subtitles of movies and TV shows using Bazarr's API

Usage: bb --config <FILE> [OPTIONS] <COMMAND>

Commands:
  movies    Perform operations on movies
  tv-shows  Perform operations on TV shows
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>     Path to the JSON configuration file [required]
      --db-path <FILE>    Path to the SQLite database file (overrides BB_DATA_DIR)
  -m, --max-retries <N>   Number of times to retry requests [default: 3]
  -r, --retry-interval <N> Retry interval in seconds [default: 10]
  -h, --help              Print help
```

### Movies

```bash
bb movies --help
# Perform operations on movies

Usage: bb --config <FILE> movies [OPTIONS] <COMMAND>

Commands:
  sync                     Sync all
  ocr-fixes                Perform OCR fixes
  common-fixes             Perform common fixes
  remove-hearing-impaired  Remove hearing impaired tags from subtitles
  remove-style-tags        Remove style tags from subtitles
  fix-uppercase            Fix uppercase subtitles
  reverse-rtl              Reverse RTL directioned subtitles
  help                     Print this message or the help of the given subcommand(s)

Options:
      --ids <IDS>        Filter records by Sonarr/Radarr ID (comma-separated)
      --offset <OFFSET>  Skip N records (ignored if ids are specified) [default: skip none] [default: 0]
      --limit <LIMIT>    Limit to N records (ignored if ids are specified) [default: unlimited]
      --skip-processed   Skip previously processed items (uses local database to track processed subtitles)
  -h, --help             Print help
```

### TV Shows

```bash
bb tv-shows --help
# Perform operations on TV shows

Usage: bb --config <FILE> tv-shows [OPTIONS] <COMMAND>

Commands:
  sync                     Sync all
  ocr-fixes                Perform OCR fixes
  common-fixes             Perform common fixes
  remove-hearing-impaired  Remove hearing impaired tags from subtitles
  remove-style-tags        Remove style tags from subtitles
  fix-uppercase            Fix uppercase subtitles
  reverse-rtl              Reverse RTL directioned subtitles
  help                     Print this message or the help of the given subcommand(s)

Options:
      --ids <IDS>        Filter records by Sonarr/Radarr ID (comma-separated)
      --offset <OFFSET>  Skip N records (ignored if ids are specified) [default: skip none] [default: 0]
      --limit <LIMIT>    Limit to N records (ignored if ids are specified) [default: unlimited]
      --skip-processed   Skip previously processed items (uses local database to track processed subtitles)
  -h, --help             Print help
```

#### Sync Options

```bash
bb movies/tv-shows sync --help
# Customize TV show/movie subtitle sync options

Usage: bb movies sync [OPTIONS]

Options:
  -r <REFERENCE>       Reference for sync from video file track number (a:0), subtitle (s:0), or some subtitles file path
  -m <MAX OFFSET>      Seconds of offset allowed when syncing [default: null]
  -n                   No fix framerate [default: false]
  -g                   Use Golden-Section search [default: false]
  -h, --help           Print help
```
