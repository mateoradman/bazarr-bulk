# BAZARR BULK CLI

A CLI tool for performing actions in bulk on Bazarr movies and tv shows.
List of supported actions:
- sync
- ocr-fixes
- common-fixes
- remove-hearing-impaired
- remove-style-tags
- reverse-rtl

## Installation

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
tar xf bazarr-bulk_*_x86_64-unknown-linux-musl.tar.gz
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
unzip bazarr-bulk_*_x86_64-apple-darwin.zip
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


## Usage
1. Create a JSON config file based on the template [file](./examples/config.json).
2. Run `bb --config your-config.json` [movies|tv-shows] [ACTION]

### General help
```bash
bb --help

Usage: bb --config <FILE> <COMMAND>

Commands:
  movies    perform operations on movies
  tv-shows  perform operations on tv shows
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>
  -h, --help           Print help
```

### Movies

```bash
bb movies --help
# perform operations on movies

Usage: bb --config <FILE> movies <COMMAND>

Commands:
  sync                     sync all
  ocr-fixes                perform OCR fixes on all
  common-fixes             perform common fixes on all
  remove-hearing-impaired  remove hearing impaired tags from subtitles
  remove-style-tags        remove style tags from subtitles
  fix-uppercase            fix uppercase subtitles
  reverse-rtl              reverse RTL directioned subtitles
  help                     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```


### TV Shows

```bash
bb tv-shows --help
# perform operations on tv shows

Usage: bb --config <FILE> tv-shows <COMMAND>

Commands:
  sync                     sync all
  ocr-fixes                perform OCR fixes on all
  common-fixes             perform common fixes on all
  remove-hearing-impaired  remove hearing impaired tags from subtitles
  remove-style-tags        remove style tags from subtitles
  fix-uppercase            fix uppercase subtitles
  reverse-rtl              reverse RTL directioned subtitles
  help                     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
