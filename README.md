# BAZARR BULK CLI

A CLI tool for performing actions in bulk on Bazarr movies and tv shows.

## Installation

## Usage

```bash
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
