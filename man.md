% NEWSCHECK(1)
% Xuanrui Qi
% 12 June 2025

## NAME

newscheck(1) -- yet another Arch Linux news reader

## SYNOPSIS

**newscheck** [check | list | read] [OPTIONS]

## DESCRIPTION

**newscheck** is an Arch Linux news reader that can also be used as a **pacman**(8) hook.
It supports: reading the news, checking for unread news items, and blocking updates if there is unread
news.

## USAGE

To check for unread news items, use:

    $ newscheck check

To list all items in the news feed, use:

    $ newscheck list

To read a specified news item, use:

    $ newscheck read [# of item]

Or, to read all news items, use:

    $ newscheck read

without any arguments.

## OPTIONS

* **-r**, **--raw**: print raw HTML in the news, instead of parsing and pretty-printing.
* **-clear-readlist**: clear the saved list of read news items. After you issue the command, all news items
will be considered unread.
* **-f [READLIST_PATH]**, **--file [READLIST_PATH]**: use an alternate path to the saved list of read news items.
The default is /var/lib/newscheck/data.
* **--url**: use an alternate endpoint for the news feed. Useful if you are using an Arch-based distribution or
due to some issue you cannot access archlinux.org.
* **-p [PAGER]**, **--pager [PAGER]**: instead of printing news items to the standard output, page them to a pager
specified in **PAGER**.
* **-h**, **--help**: print the help message.
* **--reverse**: only for **newscheck list**. List the news items in reverse (old-to-new) order.
* **--unread**: only for **newscheck list**. List only unread news items.
* **--all**: only for **newscheck read**. Mark all news items read without actually printing (and reading) them.

## ENVIRONMENT VARIABLES

Alternately, one can set the **$NEWSCHECK_PAGER** environment variable to specify a pager to use. However, one specified via
command line flags takes precedence.

## BUGS

If you find any bugs in the software, please report them at: https://github.com/xuanruiqi/newscheck/issues.

## AUTHOR

Xuanrui Qi (me@xuanruiqi.com)

## LICENSE & COPYRIGHT

The software is licensed for public use and reproduction under the terms of the MIT License. The author owns the copyright 
to and reserves all rights.