<div align="center">

# 松葉 matsuba

lightweight japanese IME written in rust

</div>

**matsuba** - a lightweight japanese ime for x written in rust. This project
has three main components:

1. kanji/word database
2. conversion system (hiragana, katakana, kanji)
3. graphical frontend for X

Each component should be separable as it's own library.

## DEPENDENCIES

- fontconfig
- xdotool (for now)

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
- **matsuba_server**: edict database wrapper, kana state machine and conversion logic
- **matsuba_common**: common types and code
- **matsuba_grpc**: tonic generated sdk for grpc

## TODO

- [x] get keycodes from xmodmap
- [x] capture all keypresses (even when not focused)
- [x] figure out how to have floating window
- [X] fetch kanji data from EDICT
- [X] set up database (probably sqlite)
- [X] arg parsing
- [ ] glyph rendering
- [x] make user type accepted string
- [x] henkan + muhenkan
- [ ] conversion menu for kanji
- [ ] refactor to daemon + cli to manage (rpc calls)
- [ ] utility cli to do kana conversions + kanji conversions
- [ ] rewrite matsuba-fetch script as a series of command calls in matsucli


