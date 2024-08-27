[![Crates.io](https://img.shields.io/crates/v/ntdsextract2)](https://crates.io/crates/ntdsextract2)
![Crates.io](https://img.shields.io/crates/l/ntdsextract2)
![Crates.io (latest)](https://img.shields.io/crates/dv/ntdsextract2)

# ntdsextract2

<img align="right" width="128px" src="https://github.com/janstarke/ntdsextract2/blob/b3ff84258487d511c6e59ab873b3db0715b4ad82/doc/images/ntdsextract2.jpeg">

This aims to be a replacement of <https://github.com/csababarta/ntdsxtract/> by @csababarta.

## Why do you write a tool that's already there and working?

1. ntdsxtract is using Python 2.7, which makes it hard to use on modern systems
1. There has been no change since a lot of time (the last commit is from February 2016), which suggests that Csaba has other stuff to do at the moment. That's OK. But Windows *is* changing, and therefore the tools to analyze Windows Systems has to adapt. As I don't like some architectural decisions Csaba has made, I started my own development.

# Installation

```bash
cargo install ntdsextract2
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

## Search for entries

```
USAGE:
    ntdsextract2 <NTDS_FILE> search [OPTIONS] <REGEX>

ARGS:
    <REGEX>    regular expression to match against

OPTIONS:
    -h, --help           Print help information
    -i, --ignore-case    case-insensitive search (ignore case)
    -q, --quiet          Less output per occurrence
    -v, --verbose        More output per occurrence
```

## Displaying a single entry

```
USAGE:
    ntdsextract2 <NTDS_FILE> entry [OPTIONS] <ENTRY_ID>

ARGS:
    <ENTRY_ID>    id of the entry to show

OPTIONS:
    -h, --help       Print help information
    -q, --quiet      Less output per occurrence
        --sid        search for SID instead for NTDS.DIT entry id. <ENTRY_ID> will be interpreted as
                     RID, wich is the last part of the SID; e.g. 500 will return the Administrator
                     account
    -v, --verbose    More output per occurrence
```

## Displaying the tree structure of the AD

```
USAGE:
    ntdsextract2 <NTDS_FILE> tree [OPTIONS]

OPTIONS:
    -h, --help                     Print help information
        --max-depth <MAX_DEPTH>    maximum recursion depth [default: 4]
    -q, --quiet                    Less output per occurrence
    -v, --verbose                  More output per occurrence
```

## Creating a timeline

```
USAGE:
    ntdsextract2 <NTDS_FILE> timeline [OPTIONS]

OPTIONS:
        --all-objects    show objects of any type (this might be a lot)
    -h, --help           Print help information
    -q, --quiet          Less output per occurrence
    -v, --verbose        More output per occurrence
```

## Enumerating ...

### ... users

```
USAGE:
    ntdsextract2 <NTDS_FILE> user [OPTIONS]

OPTIONS:
    -A, --show-all           show all non-empty values. This option is ignored when CSV-Output is
                             selected
    -F, --format <FORMAT>    Output format [default: csv] [possible values: csv, json, json-lines]
    -h, --help               Print help information
    -q, --quiet              Less output per occurrence
    -v, --verbose            More output per occurrence
```

### ... groups

```
USAGE:
    ntdsextract2 <NTDS_FILE> group [OPTIONS]

OPTIONS:
    -A, --show-all           show all non-empty values. This option is ignored when CSV-Output is
                             selected
    -F, --format <FORMAT>    Output format [default: csv] [possible values: csv, json, json-lines]
    -h, --help               Print help information
    -q, --quiet              Less output per occurrence
    -v, --verbose            More output per occurrence
```

### ... computers

```
USAGE:
    ntdsextract2 <NTDS_FILE> computer [OPTIONS]

OPTIONS:
    -A, --show-all           show all non-empty values. This option is ignored when CSV-Output is
                             selected
    -F, --format <FORMAT>    Output format [default: csv] [possible values: csv, json, json-lines]
    -h, --help               Print help information
    -q, --quiet              Less output per occurrence
    -v, --verbose            More output per occurrence
```

### ... types

```
USAGE:
    ntdsextract2 <NTDS_FILE> types [OPTIONS]

OPTIONS:
    -F, --format <FORMAT>    Output format [default: csv] [possible values: csv, json, json-lines]
    -h, --help               Print help information
    -q, --quiet              Less output per occurrence
    -v, --verbose            More output per occurrence
```
