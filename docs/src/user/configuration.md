# Configuration

**mastuba** is configured with [toml](https://toml.io/en/). The default configuration file is installed at `/usr/share/matsuba/matsuba_default.toml`. You can copy it and create per user changes:
```
cp /usr/share/matsuba/matsuba_default.toml ~/.config/matsuba/matsuba.toml
```

Currently the places **matsuba** looks for configuration files are (in increasing presedence):
- `/usr/share/matsuba/matsuba_default.toml`
- `~/.config/matsuba_.toml`

## Configuration Options

| key | purpose | default value |
|-----|---------|---------------|
| server.listen_address | port the gRPC server listens on | [::1]:10000 |
| keys.henkan | switch to henkan mode | C-comma |
| keys.muhenkan | switch to muhenkan mode | C-period |
| keys.accept | accept the currently selected conversion | Return |
| keys.delete | delete one character in conversion | BackSpace |
| keys.cancel | cancel the entire conversion | Escape |
| keys.next\_conversion | cycle to the next conversion | Tab |
| keys.prev\_conversion | cycle to the previous conversion | S-Tab |
| theme.bg | default background color | |
| theme.fg | default foreground color | |
| theme.selected\_bg | background color of selected conversion | |
| theme.selected\_fg | foreground color of selected conversion | |
| theme.completion\_bg | background color of completion | |
| theme.completion\_fg | foreground color of completion | |
| database.cache\_dir | file directory that database files will be stored in | $HOME/.config/matsuba |
