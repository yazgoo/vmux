<img src="vmux.png" alt="vmux logo" width="200"/>

[![Discord](https://img.shields.io/badge/discord--blue?logo=discord)](https://discord.gg/F684Y8rYwZ)
[![Crates.io](https://img.shields.io/crates/v/vmux?style=flat-square)](https://crates.io/crates/vmux)

Helper to use vim/neovim as a terminal multiplexer

# video demos

<table>
<tr>
Neovim conf 2022
<th>
<tr>
session switching:
<th>
<tr>
CLI + functionalities inside vim:
<th>
</tr>
<tr>
<td>
<a href=https://www.twitch.tv/videos/1675449848?t=02h59m07s><img src=doc/NeovimConf/conf/wallpapers/NeovimConf.png width=200/></a>
</td>
<td>
<a href=https://www.youtube.com/watch?v=TIZZL5dFtQc><img src=https://img.youtube.com/vi/TIZZL5dFtQc/0.jpg width=200/></a>
</td>
<td>
<a href=https://www.youtube.com/watch?v=CnLlT0Wd_wY><img src=https://img.youtube.com/vi/CnLlT0Wd_wY/0.jpg width=200/></a>
</td>
</tr>
</table>

# test it now with docker

```bash
docker run -it yazgoo/vmux:master
```

see [interactive usage](#interactive-usage) for more info on how to use it.

A full example of actual installation/customization can be found in [Dockerfile](docker/Dockerfile).

# install 

You will need rust and cargo [installed](https://www.rust-lang.org/tools/install).

Install the following vim plugin (e.g. here with vimplug), with a hook to install vmux crate: 

```vimscript
Plug 'yazgoo/vmux', {'do': 'cargo install vmux' }
```

Add the following to your `.zshrc` or `.bashrc` (replace `<your_editor>` with vim or nvim (default)).
For vim you'll need it compiled with `+clientserver` flag:

```bash
source ~/.config/nvim/plugged/vmux/plugin/setup_vmux.sh <your_editor>
```

# usage

## interactive usage

See [video demo](https://www.youtube.com/watch?v=TIZZL5dFtQc). `vmux new` will start vmux in interactive mode.

You'll be prompted to:

- create a new session (via `New: ...` (pre-named), or `New` (custom-named))
- exit (via `Detach`)
- open an existing session

You can leave current session with `CTRL+g`.
(you can change default escape key from `CTRL+g` (with `-e a`) to `CTRL+a` ).

## usage within vim / neovim

See [video demo](https://www.youtube.com/watch?v=TIZZL5dFtQc).

Within vim, vmux provides integration between vim and terminal.
Run `:help vmux` from within vim for more [in depth help](doc/vmux.txt).
see [docker/init.vim](docker/init.vim) for an example of configuration.

## cli usage

you can also manage sessions from the CLI: 

- `vmux new <session_name>` to create a new session name
- `vmux list` to list session names
- `vmux attach <full_session_name>` to attach to a running session (as per `vmux list`)

# customizing

For an optimal experience, you should at least add 
`list_sessions_names` and `session_name` hook files described below.

Both files must be executable (if they are a script, they should have a [shebang](https://en.wikipedia.org/wiki/Shebang_(Unix))).

## session setup

You can define a custom way to setup a new session via `~/.config/vmux/hooks/session_name`,
which takes the session name as argument.
The script just needs to print environment variables of the form `key=value` (`env` command will do that).

For example, [this script](docker/session_name) will print the content of `.envrc`
and set working directory to `~/dev/$1` (via `PWD` line).

## list sessions names

You can define a list of new session names via `~/.config/vmux/hooks/list_sessions_names`
The script just needs to output session names one by line, see [docker](docker/list_sessions_names) for an example.

## wallpaper

You can put images which will be used as wallpapers inside `~/.config/vmux/wallpapers/`.

## crates it relies on

This project relies on the following fundamentals crates:

- wallpapers are displayed via [blockish](https://github.com/yazgoo/blockish/)
- fuzzy prompting is done via [skim](https://github.com/lotabout/skim/)
- terminal session management: [diss](https://github.com/yazgoo/diss)
- historized ordering of selections (BAcked Up Sorter) : [baus](https://github.com/yazgoo/baus)
