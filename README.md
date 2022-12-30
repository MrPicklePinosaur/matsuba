<div align="center">

# 松葉 matsuba

lightweight japanese IME written in rust

[![book](https://img.shields.io/badge/book-website-orange)](https://mrpicklepinosaur.github.io/matsuba/)
[![build](https://github.com/MrPicklePinosaur/matsuba/workflows/Release/badge.svg)](https://github.com/MrPicklePinosaur/matsuba/actions)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

**matsuba** - a lightweight japanese ime for x written in rust. This project
has three main components:

1. kanji/word database
2. conversion system (hiragana, katakana, kanji)
3. graphical frontend for X

Each component should be separable as it's own library. Please see the
[documentation](https://mrpicklepinosaur.github.io/matsuba/) for
detailed installation, configuration and usage instructions.

## INSTALLATION

Currently **matsuba** only supports X11 (will expand to other platforms in
future!). There are a couple of utilities that need to be installed for
**matsuba** to interface with X properly.
- xdotool
- xmodmap
- sqlite3

**matsuba** is packaged for the following linux distributions
- arch linux (and friends) via AUR

## FETCHING DICTIONARY

List of word conversions come from the [EDICT/JMICT
project](https://www.edrdg.org/jmdict/edict.html). Specifically, the
[JMdict_e](http://ftp.edrdg.org/pub/Nihongo/JMdict_e.gz) (JMdict
english-japanese only) xml file is fetched. A script is provided to populate
your dictionary, along with some options on what word lists to fetch (bio,
sports, names etc), you can see a full list of 'filters'
[here](https://www.edrdg.org/jmdictdb/cgi-bin/edhelp.py?svc=jmdict&sid=#kw_fld).

## RUNNING FOR DEVELOPMENT
To run the daemon
```
just matsud
```

To run the cli
```
just matsucli
```

## PROJECT STRUCTURE

The project is broken up into the following crates
- **matsuba_cli**: end user cli for managing the matsuba daemon as well as for running conversions from the shell
- **matsuba_server**: edict database wrapper, kana state machine and conversion graphical wgpu-based frontend for displaying completions
- **matsuba_common**: common types and code
- **matsuba_grpc**: tonic generated sdk for grpc

## TODO

Roadmap to 1.0 release
- [x] get keycodes from xmodmap
- [x] capture all keypresses (even when not focused)
- [x] figure out how to have floating window
- [X] fetch kanji data from EDICT
- [X] set up database (probably sqlite)
- [X] arg parsing
- [x] glyph rendering
- [x] make user type accepted string
- [x] henkan + muhenkan
- [x] conversion menu for kanji
- [x] refactor to daemon + cli to manage (rpc calls)
- [x] utility cli to do kana conversions + kanji conversions + fetch daemon state
- [x] rewrite matsuba-fetch script as a series of command calls in matsucli
- [ ] pass through certain keys + general qol improvements
- [x] config file
- [ ] systemd, openrc and runit services for matsubad
- [x] package for various linux distributions (could use workflow to automatically build and package)

## CONTRIBUTING

Feel free to open any issues and pull requests to support development :)

