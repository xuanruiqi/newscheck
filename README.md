# newscheck

Yet another Arch Linux news reader, similar to [informant](https://github.com/bradford-smith94/informant).

`newscheck` is meant to be more or less a drop-in replacement for `informant`.

## Installation

You should install this using `pacman` from the AUR (WIP). Otherwise you could simply use `cargo` to build the binary.

## Commands

* `newscheck check`: check if there are unread news items.
* `newscheck list`: list all recent news items.
* `newscheck read`: read a specified news item.

For more information, see `newscheck --help`.

## Configuration

Unlike `informant`, `newscheck` does NOT support checking for multiple news feeds. One can specify an alternate news feed endpoint (useful
for, e.g., users located in mainland China), but it supports retrieving only from one feed. There is no plan to support this feature in the
near future either. So, there is no configuration involved.

## Differences from Informant

Some functionalities that seem to be rarely used are omitted. These include:

* the ability to check for multiple news feeds;
* support for HTTP request caching.

There are some subtle behavior differences with `informant` that are conscious choices. For example:

* when `--reverse` is given, the numerical ID of each news item is printed in reverse order too;
* when pretty-printing news items using `newscheck read`, the text wraps to current terminal width, rather than 80 characters;
* timestamps are in the local time zone.

But these are mainly cosmetic differences.
