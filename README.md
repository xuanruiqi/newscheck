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
