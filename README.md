![Crates.io](https://img.shields.io/crates/v/ntdsextract2)
![Crates.io](https://img.shields.io/crates/l/ntdsextract2)
![Crates.io (latest)](https://img.shields.io/crates/dv/ntdsextract2)

# ntdsextract2

This aims to be a replacement of <https://github.com/csababarta/ntdsxtract/> by @csababarta.

## Why do you write a tool that's already there and working?

1. ntdsxtract is using Python 2.7, which makes it hard to use on modern systems
1. There has been no change since a lot of time (the last commit is from February 2016), which suggests that Csaba has other stuff to do at the moment. That's OK. But Windows *is* changing, and therefore the tools to analyze Windows Systems has to adapt. As I don't like some architectural decisions Csaba has made, I started my own development.

# Installation

```bash
cargo install --git https://github.com/janstarke/ntdsextract2.git
```

# Usage
```
USAGE:
    ntdsextract2 [OPTIONS] <NTDS_FILE> <SUBCOMMAND>

ARGS:
    <NTDS_FILE>    name of the file to analyze

OPTIONS:
    -h, --help       Print help information
    -q, --quiet      Less output per occurrence
    -v, --verbose    More output per occurrence
    -V, --version    Print version information

SUBCOMMANDS:
    computer    display computer accounts
    entry       display one single entry from the directory information tree
    group       Display groups
    help        Print this message or the help of the given subcommand(s)
    search      search for entries whose values match to some regular expression
    timeline    create a timeline (in bodyfile format)
    tree        display the directory information tree
    types       list all defined types
    user        Display user accounts

```