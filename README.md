
# 松葉matsuba

**matsuba** - a lightweight japanese ime for x written in rust. This project
has three main components:

1. kanji/word database
2. conversion system (hiragana, katakana, kanji)
3. graphical frontend for X

Each component should be separable as it's own library.

## DEPENDENCIES

- fontconfig

## FETCHING DICTIONARY

List of word conversions come from the [EDICT/JMICT
project](https://www.edrdg.org/jmdict/edict.html). Specifically, the
[JMdict_e](http://ftp.edrdg.org/pub/Nihongo/JMdict_e.gz) (JMdict
english-japanese only) xml file is fetched. A script is provided to populate
your dictionary, along with some options on what word lists to fetch (bio,
sports, names etc), you can see a full list of 'filters'
[here](https://www.edrdg.org/jmdictdb/cgi-bin/edhelp.py?svc=jmdict&sid=#kw_fld).

## RUNNING FOR DEVELOPMENT

```
 $ cargo run
```

## TODO

- [x] get keycodes from xmodmap
- [ ] capture all keypresses (even when not focused)
- [ ] figure out how to have floating window
- [ ] fetch kanji data from EDICT
- [ ] set up database (probably sqlite)
- [ ] arg parsing

## RESOURCES

couple resources that were used when writing this project


