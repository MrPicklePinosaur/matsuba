# 使い方

## 機動

**matsud**とは**matsuba**のバックエンドです。データベースアクセスなり平仮名変化なりを頼みます。言うまでもないが**matsuba**を使ったら**matsud**が必要です。ヂィーモンとして実行する:
```sh
matsud &
```
その上systemdで管理も可能です:
```sh
sudo systemctl enable matsuba
sudo systemctl start matsuba
```

## 漢字辞書をダウンロード

**matsuba**を使い前に、漢字辞書が不可欠です。以下のコマンッドは[EDICT/JMICT project](https://www.edrdg.org/jmdict/edict.html)から漢字辞書をダウンロードして、データベースに挿入します:
```sh
matsucli fetch
```
You can also fine tune which kanji lists are populated via tags/filters. By default all tags are enabled. A full list can be found [here](https://www.edrdg.org/jmdictdb/cgi-bin/edhelp.py?svc=jmdict&sid=#kw_fld). Just pass in which tags you wish to include (or not include) using the `tag` flag:

例えば野球と植物学の語彙を省いたが、ギリシャ神話の語彙を含めます。
```sh
matsucli fetch --tag -baseb,-bot,+grmyth
```

## matsucli

**matsucli** is a utility command line interface to interact with the main **matsuba** daemon, **matsud**. **matsucli** let's you query and modify the state of **matsuba**, such as enabling and disabling henkan mode, as well as converting kana. This is great for interacting with scripts.
