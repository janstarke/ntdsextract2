[![Crates.io](https://img.shields.io/crates/v/ntdsextract2)](https://crates.io/crates/ntdsextract2)
![Crates.io](https://img.shields.io/crates/l/ntdsextract2)
![Crates.io (latest)](https://img.shields.io/crates/dv/ntdsextract2)

- [ntdsextract2](#ntdsextract2)
  - [Why do you write a tool that's already there and working?](#why-do-you-write-a-tool-thats-already-there-and-working)
- [Installation](#installation)
- [Usage](#usage)
  - [Search for entries](#search-for-entries)
  - [Displaying a single entry](#displaying-a-single-entry)
  - [Displaying the tree structure of the AD](#displaying-the-tree-structure-of-the-ad)
  - [Creating a timeline](#creating-a-timeline)
  - [Enumerating ...](#enumerating-)
    - [... users](#-users)
    - [... groups](#-groups)
    - [... computers](#-computers)
    - [... types](#-types)
  - [Configuring the global timestamp format](#configuring-the-global-timestamp-format)
- [Forensics details](#forensics-details)
  - [Interpreting timestamps](#interpreting-timestamps)

# ntdsextract2

<img align="right" width="128px" src="https://raw.githubusercontent.com/janstarke/ntdsextract2/main/doc/images/ntdsextract2.jpeg">

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


## Configuring the global timestamp format

Per default, `ntdsextract2` uses an RFC3339-compliant data format. If you want to, you can change the data format
being used by setting the `DFIR_DATE` environment variable. Let's look at an example:

```shell
$ ntdsextract2 tests/data/ntds_plain.dit user -F json-lines |jq 'select (.rdn == "Administrator")'
```

```json
{
  "sid": "S-1-5-21-1467604378-2733498025-3532005688-500",
  "user_principal_name": null,
  "rdn": "Administrator",
  "sam_account_name": "Administrator",
  "sam_account_type": "SAM_USER_OBJECT",
  "user_account_control": "ADS_UF_NORMAL_ACCOUNT | ADS_UF_DONT_EXPIRE_PASSWD",
  "logon_count": 4,
  "bad_pwd_count": 0,
  "admin_count": null,
  "is_deleted": false,
  "primary_group_id": 513,
  "primary_group": "Domänen-Benutzer",
  "member_of": [
    "Richtlinien-Ersteller-Besitzer",
    "Schema-Admins",
    "Administratoren",
    "Organisations-Admins",
    "Domänen-Admins"
  ],
  "comment": null,
  "record_time": "2023-11-15T06:33:44+0000",
  "when_created": "2023-11-15T06:33:44+0000",
  "when_changed": "2023-11-15T06:41:50+0000",
  "last_logon": "2023-11-15T06:41:50+0000",
  "last_logon_time_stamp": "2023-11-15T06:41:50+0000",
  "account_expires": "+30828-09-14T02:48:05+0000",
  "password_last_set": "2023-11-15T05:40:32+0000",
  "bad_pwd_time": "1601-01-01T00:00:00+0000"
}
```


```shell
$ DFIR_DATE="%F %T (%Z)" ntdsextract2 tests/data/ntds_plain.dit user -F json-lines |jq 'select (.rdn == "Administrator")'
```

```json
{
  "sid": "S-1-5-21-1467604378-2733498025-3532005688-500",
  "user_principal_name": null,
  "rdn": "Administrator",
  "sam_account_name": "Administrator",
  "sam_account_type": "SAM_USER_OBJECT",
  "user_account_control": "ADS_UF_NORMAL_ACCOUNT | ADS_UF_DONT_EXPIRE_PASSWD",
  "logon_count": 4,
  "bad_pwd_count": 0,
  "admin_count": null,
  "is_deleted": false,
  "primary_group_id": 513,
  "primary_group": "Domänen-Benutzer",
  "member_of": [
    "Administratoren",
    "Schema-Admins",
    "Domänen-Admins",
    "Organisations-Admins",
    "Richtlinien-Ersteller-Besitzer"
  ],
  "comment": null,
  "record_time": "2023-11-15 06:33:44 (UTC)",
  "when_created": "2023-11-15 06:33:44 (UTC)",
  "when_changed": "2023-11-15 06:41:50 (UTC)",
  "last_logon": "2023-11-15 06:41:50 (UTC)",
  "last_logon_time_stamp": "2023-11-15 06:41:50 (UTC)",
  "account_expires": "+30828-09-14 02:48:05 (UTC)",
  "password_last_set": "2023-11-15 05:40:32 (UTC)",
  "bad_pwd_time": "1601-01-01 00:00:00 (UTC)"
}
```

See the difference?

# Forensics details

## Interpreting timestamps

Active Directory stores its timestamp values mostly as [`JET_coltypCurrency`](https://learn.microsoft.com/en-us/windows/win32/extensible-storage-engine/jet-coltyp), because it is the only supported 64bit fixed integer value. So, in fact, timestamps are stored as signed 64bit values. This leaves enough space to use the [`FILETIME`](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/filetime) structure.

This structure specifies the number of 100 nanoseconds since January 1, 1601. The minimum value is - of course - *`1601-01-01T00:00:00`* with a binary value of `0x0000000000000000`. The maximum value in the most cases is `0x7FFFFFFFFFFFFFFF`, which corresponds to the value *`30828-09-14T02:48:05.4775807`*. However, it is up to each software to interpret those special values.

The following table shows how each of the AD attributes are to be interpreted:

|Attribute| Interpretation of `0x0000000000000000` | Interpretation of `0x7FFFFFFFFFFFFFFF` | 
|-|----|----|
`record_time`| nothing specific |nothing specific|
`when_created`| nothing specific |nothing specific|
`when_changed`| nothing specific | nothing specific|
[`last_logon`](https://learn.microsoft.com/en-us/windows/win32/adschema/a-lastlogon)| last logon time is unknown | |
[`last_logon_time_stamp`](https://learn.microsoft.com/en-us/windows/win32/adschema/a-lastlogontimestamp)| nothing specific but likely related to above | |
[`account_expires`](https://learn.microsoft.com/en-us/windows/win32/adschema/a-accountexpires)| "If at any point in time an account which was configured with an expiration time is set back to Never Expires, the **accountExpires** attribute is then set to 0." |"When an account is created, the account is initially set to Never Expire. The **accountExpires** attribute is set to the default of *9223372036854775807*, a value which corresponds the maximum value of a 64-bit signed integer." | 
`password_last_set`| if UAC attr does not contain `UF_DONT_EXPIRE_PASSWD` then user must change password at next logon| |
[`bad_pwd_time`](https://learn.microsoft.com/en-us/windows/win32/adschema/a-badpasswordtime)| the last time an incorrect password was used is unknown| |