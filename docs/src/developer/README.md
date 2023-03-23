# Developer Guide

**matsuba** encourages the creation of 3rd party completion renderers (the little box that pops up when you start typing).

An in-depth developer guide is coming soon...

## Getting started for development

First clone the repository:
```
git clone https://github.com/MrPicklePinosaur/matsuba
```

You will need the [just command runner](https://github.com/casey/just). To
install git hooks for auto-formatting, linting and more:
```
just devsetup
```

To run the daemon
```
just matsud
```

To run the cli
```
just matsucli
```

## Project Structure

The project is broken up into the following crates
- **matsuba_cli**: end user cli for managing the matsuba daemon as well as for running conversions from the shell
- **matsuba_server**: edict database wrapper, kana state machine and conversion graphical wgpu-based frontend for displaying completions
- **matsuba_common**: common types and code
- **matsuba_grpc**: tonic generated sdk for grpc

