# Usage

## Starting matsud

**matsud** is the actual server that handles the heaving lifting. You can start it as a daemon by running
```sh
matsud &
```
Alternatively, you can use your init system to automatically manage **matsud** for you. For systemd users:
```sh
sudo systemctl enable matsuba
sudo systemctl start matsuba
```

## Fetching Dictionary

**matsuba** first needs to fetch kanji lists from the [EDICT/JMICT project](https://www.edrdg.org/jmdict/edict.html) for use in kanji completions. You can populate the **matsuba** database by running
```sh
matsucli fetch
```
You can also fine tune which kanji lists are populated via tags/filters. By default all tags are enabled. A full list can be found [here](https://www.edrdg.org/jmdictdb/cgi-bin/edhelp.py?svc=jmdict&sid=#kw_fld). Just pass in which tags you wish to include (or not include) using the `tag` flag:
```sh
matsucli fetch --tag -baseb,-bot,+grmyth
```
In the above example, baseball words and botany words are not included, but greek myth words are.

## matsucli

**matsucli** is a utility command line interface to interact with the main **matsuba** daemon, **matsud**. **matsucli** let's you query and modify the state of **matsuba**, such as enabling and disabling henkan mode, as well as converting kana. This is great for interacting with scripts.
