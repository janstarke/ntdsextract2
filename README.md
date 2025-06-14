> [!CAUTION]
> # `ntdsextract2` is leaving github
> 
> ## Why?
>
> I believe that all men (and women, and all human between and above) are created equal. In this mindset, it does not make sense to judge people based on their birthplace, or their language, color, religion, or whatsoever.
> 
> I believe that who you are is made up of what you do. If you are caring towards other people, then that's you are. If you do harm to other people, then that's who you are.
> 
> I'm concerned of what is currently happening in the United States. I don't like it when a government thinks it is above the law. I don't like it when a government doesn't serve the people, but sees people as a threat. But that's politics.
> 
> Github is part of Microsoft, and Microsoft is supporting this government. For example, Microsoft blocked the mail accounts of ICC members because of political reasons. I don't want to get my accounts blocked or deleted arbitrarily. Therefore, I'going to not support Microsoft in any way. That's why I'll move all my repositories away from github.
> 
> We had a good time. Cheers.
>
> ## Where?
> 
> The new place-to-be for `ntdsextract2` is <https://codeberg.org/janstarke/ntdsextract2>.

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
Usage: ntdsextract2 [OPTIONS] <NTDS_FILE> <COMMAND>

Commands:
  user      Display user accounts
  group     Display groups
  computer  display computer accounts
  timeline  create a timeline (in bodyfile format)
  types     list all defined types
  tree      display the directory information tree
  entry     display one single entry from the directory information tree
  search    search for entries whose values match to some regular expression
  help      Print this message or the help of the given subcommand(s)

Arguments:
  <NTDS_FILE>  name of the file to analyze

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```

## Search for entries

```
Usage: ntdsextract2 <NTDS_FILE> search [OPTIONS] <REGEX>

Arguments:
  <REGEX>  regular expression to match against

Options:
  -i, --ignore-case  case-insensitive search (ignore case)
  -v, --verbose...   Increase logging verbosity
  -q, --quiet...     Decrease logging verbosity
  -h, --help         Print help
```

## Displaying a single entry

```
Usage: ntdsextract2 <NTDS_FILE> entry [OPTIONS] <ENTRY_ID>

Arguments:
  <ENTRY_ID>
          id of the entry to show

Options:
      --sid
          search for SID instead for NTDS.DIT entry id. <ENTRY_ID> will be interpreted as RID, wich is the last part of the SID; e.g. 500 will return the Administrator account

  -F, --format <ENTRY_FORMAT>
          [default: simple]

          Possible values:
          - json:   use JSON format
          - table:  display a formatted table
          - simple: use a simple key-values based format

  -v, --verbose...
          Increase logging verbosity

  -q, --quiet...
          Decrease logging verbosity

  -h, --help
          Print help (see a summary with '-h')
```

## Displaying the tree structure of the AD

```
Usage: ntdsextract2 <NTDS_FILE> tree [OPTIONS]

Options:
      --max-depth <MAX_DEPTH>  maximum recursion depth [default: 4]
  -v, --verbose...             Increase logging verbosity
  -q, --quiet...               Decrease logging verbosity
  -h, --help                   Print help
```

## Creating a timeline

```
Usage: ntdsextract2 <NTDS_FILE> timeline [OPTIONS]

Options:
      --all-objects      show objects of any type (this might be a lot)
      --include-deleted  include also deleted objects (which don't have an AttObjectCategory attribute)
  -v, --verbose...       Increase logging verbosity
  -q, --quiet...         Decrease logging verbosity
  -h, --help             Print help
```

## Enumerating ...

### ... users

```
Usage: ntdsextract2 <NTDS_FILE> user [OPTIONS]

Options:
  -F, --format <FORMAT>
          Output format
          
          [default: csv]
          [possible values: csv, json, json-lines]

  -A, --show-all
          show all non-empty values. This option is ignored when CSV-Output is selected

  -D, --include-dn
          include the distinguished name (DN) in the output.
          
          Note that this property is not an attribute of the AD entry iself; instead it is constructed from the relative DN (RDN) of the entry and all of its parents. That's why this property is normally not shown.

      --member-of <MEMBER_OF_ATTRIBUTE>
          specify which attribute shall be used to display group memberships
          
          [default: rdn]

          Possible values:
          - sid: show the Security ID (SID)
          - rdn: show the relative distinguished name (RDN) value
          - dn:  show the distinguished name (DN)
          - sam: show the samAccountName attribute


  -v, --verbose...
          Increase logging verbosity

  -q, --quiet...
          Decrease logging verbosity

  -h, --help
          Print help (see a summary with '-h')
```

### ... groups

```
Usage: ntdsextract2 <NTDS_FILE> group [OPTIONS]

Options:
  -F, --format <FORMAT>
          Output format
          
          [default: csv]
          [possible values: csv, json, json-lines]

  -A, --show-all
          show all non-empty values. This option is ignored when CSV-Output is selected

  -D, --include-dn
          include the distinguished name (DN) in the output.
          
          Note that this property is not an attribute of the AD entry iself; instead it is constructed from the relative DN (RDN) of the entry and all of its parents. That's why this property is normally not shown.

      --member-of <MEMBER_OF_ATTRIBUTE>
          specify which attribute shall be used to display group memberships
          
          [default: rdn]

          Possible values:
          - sid: show the Security ID (SID)
          - rdn: show the relative distinguished name (RDN) value
          - dn:  show the distinguished name (DN)
          - sam: show the samAccountName attribute

  -v, --verbose...
          Increase logging verbosity

  -q, --quiet...
          Decrease logging verbosity

  -h, --help
          Print help (see a summary with '-h')
```

### ... computers

```
Usage: ntdsextract2 <NTDS_FILE> computer [OPTIONS]

Options:
  -F, --format <FORMAT>
          Output format
          
          [default: csv]
          [possible values: csv, json, json-lines]

  -A, --show-all
          show all non-empty values. This option is ignored when CSV-Output is selected

  -D, --include-dn
          include the distinguished name (DN) in the output.
          
          Note that this property is not an attribute of the AD entry iself; instead it is constructed from the relative DN (RDN) of the entry and all of its parents. That's why this property is normally not shown.

      --member-of <MEMBER_OF_ATTRIBUTE>
          specify which attribute shall be used to display group memberships
          
          [default: rdn]

          Possible values:
          - sid: show the Security ID (SID)
          - rdn: show the relative distinguished name (RDN) value
          - dn:  show the distinguished name (DN)
          - sam: show the samAccountName attribute

  -v, --verbose...
          Increase logging verbosity

  -q, --quiet...
          Decrease logging verbosity

  -h, --help
          Print help (see a summary with '-h')
```

### ... types

```
Usage: ntdsextract2 <NTDS_FILE> types [OPTIONS]

Options:
  -F, --format <FORMAT>  Output format [default: csv] [possible values: csv, json, json-lines]
  -v, --verbose...       Increase logging verbosity
  -q, --quiet...         Decrease logging verbosity
  -h, --help             Print help
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

Active Directory stores its timestamp values mostly as [`JET_coltypCurrency`](https://learn.microsoft.com/en-us/windows/win32/extensible-storage-engine/jet-coltyp), because it is the only supported 64bit fixed integer value. So, in fact, timestamps are stored as unsigned 64bit values. This leaves enough space to use the [`FILETIME`](https://learn.microsoft.com/en-us/office/client-developer/outlook/mapi/filetime) structure.

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