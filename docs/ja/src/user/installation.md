# インストール方法

## リナックス

### アーチ・リナックス

既製リリースはAURからインストールできる:
```
yay -S matsuba-bin
```
むしろ直近のコミットからビルドしたい方は:
```
yay -S matsuba-git
```


## ソースからビルド

まずリポジトリをクローンして:
```sh
git clone https://github.com/MrPicklePinosaur/matsuba
```

それでビルド:
```sh
cargo build --release
```
