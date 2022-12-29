# Configuration

**mastuba** is configured with [toml](https://toml.io/en/). The default configuration file is installed at `/usr/share/matsuba/matsuba_default.toml`. You can copy it and create per user changes:
```
cp /usr/share/matsuba/matsuba_default.toml ~/.config/matsuba/matsuba.toml
```

Currently the places **matsuba** looks for configuration files are (in increasing presedence):
- `/usr/share/matsuba/matsuba_default.toml`
- `~/.config/matsuba_.toml`
