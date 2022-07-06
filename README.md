# ntdsforensics
This aims to be a collection of tools to forensically analyze Active Directory databases

# Usage
```
ntdsextract2 0.1.0

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
    help        Print this message or the help of the given subcommand(s)
    timeline    create a timeline (in bodyfile format)
    user        Display user accounts
```