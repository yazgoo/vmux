<img src="vmux.png" alt="vmux logo" width="200"/>

[![Discord](https://img.shields.io/badge/discord--blue?logo=discord)](https://discord.gg/F684Y8rYwZ)
[![Crates.io](https://img.shields.io/crates/v/vmux?style=flat-square)](https://crates.io/crates/vmux)

Helper to use vim/neovim as a terminal multiplexer

# video demos

<table>
<tr>
<th>
Neovim conf '22 presentation
</th>
<th>
session switching
</th>
<th>
CLI + functionalities inside vim
</th>
</tr>
<tr>
<td>
<a href=https://www.youtube.com/watch?v=CxYBBnYsY0Y>
<img 
title="presentation of vmux in Neovim conf 2022.
(Image extracted from https://www.neovimconf.live)"
src=doc/NeovimConf/conf/wallpapers/NeovimConf.png 
width=200/></a>
</td>
<td>
<a href=https://www.youtube.com/watch?v=TIZZL5dFtQc>
<img
title="describes how to switch sessions interactively"
src=https://img.youtube.com/vi/TIZZL5dFtQc/0.jpg 
width=200/>
</a>
</td>
<td>
<a href=https://www.youtube.com/watch?v=CnLlT0Wd_wY>
<img
title="how to use vmux CLI and vmux integration within terminal"
src=https://img.youtube.com/vi/CnLlT0Wd_wY/0.jpg width=200/></a>
</td>
</tr>
</table>

# docker demo

`docker run -it yazgoo/vmux:master` ([more info on how to use it](#interactive-usage),  based on [Dockerfile](docker/Dockerfile)) 

# install 

You will need rust and cargo [installed](https://www.rust-lang.org/tools/install).

Install the plugin with post-update hook to install vmux crate:

<details>
  <summary>with vim-plug</summary>
  <br>
  
  ```vim
  Plug 'yazgoo/vmux', {'do': 'cargo install vmux' }
  ```
</details>

<details>
  <summary>with lazy.nvim</summary>
  <br>
  
  ```lua
  { 'yazgoo/vmux', build = 'cargo install vmux' }
  ```
</details>

<details>
  <summary>with packer.nvim</summary>
  <br>
  
  ```lua
  use { 'yzgoo/vmux', run = 'cargo install vmux' }
  ```
</details> 

Add the following to your `.zshrc` or `.bashrc` (replace `<your_editor>` with vim or nvim (default)).<br/>
(For vim you'll need it compiled with `+clientserver` flag)

<details>
  <summary>with vim-plug</summary>
  <br>
  
  ```bash
  source ~/.config/nvim/plugged/vmux/plugin/setup_vmux.sh <your_editor>
  ```
</details>
  
<details>
  <summary>with lazy.nvim</summary>
  <br>
  
  ```bash
  source ~/.local/share/nvim/lazy/vmux/plugin/setup_vmux.sh <your_editor>
  ```
</details>
<details>
  <summary>with packer.nvim</summary>
  <br>
  
  ```bash
  source ~/.local/share/nvim/site/pack/packer/start/vmux/plugin/setup_vmux.sh <your_editor>
  ```
</details>

**Note**: if you are using Fish shell, consider using [bass](https://github.com/edc/bass) with the following added to `config.fish`:

```fish
bass source <path-to-setup_vmux.sh> <your_editor>
```

# usage

### interactive usage

[video demo](https://www.youtube.com/watch?v=TIZZL5dFtQc)

`vmux new` will start vmux in interactive mode. You'll be prompted to:

- create a new session (via `New: ...` (pre-named), or `New` (custom-named))
- exit (via `Detach`)
- open an existing session

You can leave current session with `CTRL+g`. (you can change default escape key from `CTRL+g` (with `-e a`) to `CTRL+a` ).

### usage within vim/neovim

[video demo](https://www.youtube.com/watch?v=TIZZL5dFtQc)

Within vim, vmux provides integration between vim and terminal.
Run [`:help vmux`](doc/vmux.txt) from within vim for more info.
[Here](docker/init.vim) is an example configuration.

### cli usage

- `vmux new <session_name>` creates a new session, 
- `vmux list` list running sessions,
- `vmux attach <full_session_name>` attaches to a running session (as per `vmux list`)
- you can group sessions with `-s` option.

# customizing

For an optimal experience, you should at least add 
`list_sessions_names` and `session_name` hook files described below.

Both files must be executable
(if they are a script, they should have a [shebang](https://en.wikipedia.org/wiki/Shebang_(Unix))).

### list sessions names

You can define a list of new session names via `~/.config/vmux/hooks/list_sessions_names`
The script just needs to output session names one per line, see [this](docker/list_sessions_names) for an example.

### session setup

You can define a custom way to setup a new session via `~/.config/vmux/hooks/session_name`.<br/>
The script takes the session name as argument and should print environment variables of the form `key=value`.

For example, [this script](docker/session_name) will print the content of `.envrc`
and set working directory to `~/dev/$1` (via `PWD` line).

### wallpaper

You can put images which will be used as wallpapers inside `~/.config/vmux/wallpapers/`.

### detach / attach callbacks

You can define callbacks on attach / detach session.
For example, to share data between (n)vim instances (including registers content),
you can use shared data files (`Shada`), by adding in your vim conf: 

```vim
function! VmuxCustomDetachCallback()
  wshada
endfunction

function! VmuxCustomAttachCallback()
  rshada
endfunction
```

you can get info about current session via the following env var:

- `VMUX_SESSION_GROUP`: session group name
- `VMUX_DISPLAY_NAME`: session display name within current group
- `VMUX_SESSION_NAME`: full session name used to identify the session uniquely
