---
sidebar_position: 5
title: geoetl-cli completions
description: Generate shell completions
---

# geoetl-cli completions

Generate shell completion scripts for command-line auto-completion.

## Synopsis

```bash
geoetl-cli completions <SHELL>
```

## Description

The `completions` command generates shell completion scripts that enable tab-completion for GeoETL commands and options in your shell.

## Supported Shells

- `bash`
- `zsh`
- `fish`
- `powershell`
- `elvish`

## Examples

### Bash

```bash
# Generate and install
geoetl-cli completions bash > ~/.local/share/bash-completion/completions/geoetl
source ~/.local/share/bash-completion/completions/geoetl
```

### Zsh

```bash
# Add to ~/.zshrc: fpath=(~/.zsh/completions $fpath)
mkdir -p ~/.zsh/completions
geoetl-cli completions zsh > ~/.zsh/completions/_geoetl
```

### Fish

```bash
geoetl-cli completions fish > ~/.config/fish/completions/geoetl.fish
```

### PowerShell

```powershell
geoetl-cli completions powershell > geoetl.ps1
# Add to your $PROFILE
```

## See Also

- [Installation Guide](../getting-started/installation.md#shell-completions-optional)
