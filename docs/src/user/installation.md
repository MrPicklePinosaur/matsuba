# Installation

## Linux Packages

**matsuba** is packaged for various linux distributions.

### Arch Linux

Avaliable on the AUR with either the built version of the last stable release
```
yay -S matsuba-bin
```
or latest commit directly from git
```
yay -S matsuba-git
```


## From Source

Clone the repository:
```sh
git clone https://github.com/MrPicklePinosaur/matsuba
```

Ensure you have the following dependencies
- cmake
- xdotool
- xmodmap
- sqlite3

Build the source:
```sh
cargo build --release
```

## Systemd

**matsuba** can be installed as a user systemd service. Since it relies on having an Xorg session, ensure that the script `/etc/X11/xinit/xinitrc.d/50-systemd-user.sh` is installed, or you can manually run
```
systemctl --user import-environment DISPLAY XAUTHORITY
```
in your `.xinitrc` or equivalent script that gets ran on X server startup.

With the service installed, just run
```
systemctl --user enable matsuba
systemctl --user start matsuba
```

**matsuba** logs can be viewed with
```
journalctl -f --user-unit matsuba
```
