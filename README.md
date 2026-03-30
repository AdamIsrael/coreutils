# coreutils

![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)

A Rust implementation of the stalwart GNU [coreutils](https://github.com/coreutils/coreutils).

## Why

This began as, and continues to be, a learning exercise to better understand the [Rust](https://www.rust-lang.org/) programming language. A 100+ program spin on the [Advent of Code](https://en.wikipedia.org/wiki/Advent_of_Code), allowing me to re-imagine a modern coreutils and explore system programming and best practices.

## Design goals

- Commands should support output formatting where applicable: plain, table, json and yaml to allow them to be more easily consumed by automation, CI, and AI.
- All commands should support standard flags: --help, --version

## TODO

- Finish refactor of completed commands (to use shared library for easier clap implementation and follow a modern approach/best practices)
- for easier testing, a way to temporarily add the project binaries into the current path.
  - add `just shell` target

## Status

| util | status |
| ---- | ------ |
| arch | :white_check_mark: |
| b2sum | :white_check_mark: |
| base32 | :white_check_mark: |
| base64 | :white_check_mark: |
| basename | :white_check_mark: |
| cat | :white_check_mark: |
| chcon | :white_large_square: |
| chgrp | :white_large_square: |
| chmod | :white_large_square: |
| chown | :white_large_square: |
| chroot | :white_large_square: |
| cksum | :white_large_square: |
| comm | :white_large_square: |
| cp | :white_large_square: |
| csplit | :white_large_square: |
| cut | :white_large_square: |
| date | :white_large_square: |
| dd | :white_large_square: |
| df | :white_large_square: |
| dir | :white_large_square: |
| dircolors | :white_large_square: |
| dirname | :white_check_mark: |
| du | :white_large_square: |
| echo | :white_check_mark: |
| env | :white_large_square: |
| expand | :white_large_square: |
| expr | :white_large_square: |
| factor | :white_large_square: |
| false | :white_large_square: |
| fmt | :white_large_square: |
| fold | :white_large_square: |
| groups | :white_large_square: |
| head | :white_large_square: |
| hostid | :white_large_square: |
| id | :white_large_square: |
| install | :white_large_square: |
| join | :white_large_square: |
| link | :white_large_square: |
| ln | :white_large_square: |
| logname | :white_large_square: |
| ls | :white_large_square: |
| md5sum | :white_large_square: |
| mkdir | :white_large_square: |
| mkfifo | :white_large_square: |
| mknod | :white_large_square: |
| mktemp | :white_large_square: |
| mv | :white_large_square: |
| nice | :white_large_square: |
| nl | :white_large_square: |
| nohup | :white_large_square: |
| nproc | :white_large_square: |
| numfmt | :white_large_square: |
| od | :white_large_square: |
| paste | :white_large_square: |
| pathchk | :white_large_square: |
| pinky | :white_large_square: |
| pr | :white_large_square: |
| printenv | :white_large_square: |
| printf | :white_large_square: |
| ptx | :white_large_square: |
| pwd | :white_large_square: |
| readlink | :white_large_square: |
| realpath | :white_large_square: |
| rm | :white_large_square: |
| rmdir | :white_large_square: |
| runcon | :white_large_square: |
| seq | :white_large_square: |
| sha1sum | :white_large_square: |
| sha224sum | :white_large_square: |
| sha256sum | :white_large_square: |
| sha384sum | :white_large_square: |
| sha512sum | :white_large_square: |
| shred | :white_large_square: |
| shuf | :white_large_square: |
| sleep | :white_large_square: |
| sort | :white_large_square: |
| split | :white_large_square: |
| stat | :white_large_square: |
| stdbuf | :white_large_square: |
| stty | :white_large_square: |
| sum | :white_large_square: |
| sync | :white_large_square: |
| tac | :white_large_square: |
| tail | :white_large_square: |
| tee | :white_large_square: |
| test | :white_large_square: |
| timeout | :white_large_square: |
| touch | :white_large_square: |
| tr | :white_large_square: |
| true | :white_large_square: |
| truncate | :white_large_square: |
| tsort | :white_large_square: |
| tty | :white_large_square: |
| uname | :white_large_square: |
| unexpand | :white_large_square: |
| uniq | :white_large_square: |
| unlink | :white_large_square: |
| uptime | :white_large_square: |
| users | :white_large_square: |
| vdir | :white_large_square: |
| wc | :white_large_square: |
| who | :white_large_square: |
| whoami | :white_large_square: |
| yes | :white_large_square: |
