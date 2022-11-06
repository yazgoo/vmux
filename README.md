<img src="vmux.png" alt="vmux logo" width="200"/>

Helper to use vim/neovim as a terminal multiplexer

# video demos

<table>
<tr>
<td>
session switching:
<br/>
<a href=https://www.youtube.com/watch?v=TIZZL5dFtQc><img src=https://img.youtube.com/vi/TIZZL5dFtQc/0.jpg width=200/></a>
</td>
<td>
CLI + functionalities inside vim:
<br/>
<a href=https://www.youtube.com/watch?v=CnLlT0Wd_wY><img src=https://img.youtube.com/vi/CnLlT0Wd_wY/0.jpg width=200/></a>
</td>
</tr>
</table>

# try it with docker

Build (will take a few minutes) and then run demo container:

```bash
docker run -it $(docker build -q docker)
```

# install 

An example of actual installation/customization can be found in [Dockerfile](docker/Dockerfile).

You will need rust and cargo [installed](https://www.rust-lang.org/tools/install).

Then install the following vim plugin, with a hook to install vmux crate: 

```vimscript
Plug 'yazgoo/vmux', {'do': 'cargo install vmux' }
```

Then add the following to your .zshrc or .bashrc

```bash
source ~/.config/nvim/plugged/vmux/plugin/setup_vmux.sh
```

Or if you want to use vim instead of nvim (you need vim compiled with `+clientserver` flag) :

```bash
source ~/.config/nvim/plugged/vmux/plugin/setup_vmux.sh vim
```

# usage

## interactive usage

`vmux new` will start vmux in interactive mode.

You'll be prompted to:

- create a new session (via `New: ...`, or `New`)
- exit (via `Detach`)
- open an existing session

You can leave current session with `CTRL+g`.
(you can change default escape key from `CTRL+a` (with `-e a`) to `CTRL+g` ).

## usage within vim / neovim

Within vim/neovim, vmux provides integration between the vim/neovim and terminal.
Run `:help vmux` from within vim for more in depth help.
see [docker/init.vim](docker/init.vim) for an example of configuration.

## cli usage

you can also manage sessions from the CLI: 

- `vmux new <session_name>` to create a new session name
- `vmux list` to list session names
- `vmux attach <full_session_name>` to attach to a running session (as per `vmux list`)

# customizing

## session setup

You can define a custom way to setup a new session via `~/.config/vmux/hooks/session_name.sh`
The script just needs to print environment variables of the form (`env` command will do that):

key=value

it takes the session name as argument.

For example, this script will print the content of envrc
and set working directory to `~/dev/$1` (via `PWD` line)

see [docker](docker/session_name.sh) for an example.

## list sessions names

You can define a list of new session names via `~/.config/vmux/hooks/list_sessions_names.sh`
The script just needs to output session names one by line.

see [docker](docker/list_sessions_names.sh) for an example.

## wallpaper

You can put images which will be used as wallpapers inside `~/.config/vmux/wallpapers/`.

## extra: tabbar

Having a nice tabbar (based on [Caagr98/c98tabbar.vim](https://github.com/Caagr98/c98tabbar.vim)):

```vimscript
Plug 'git@github.com:yazgoo/c98tabbar.vim'
Plug 'yazgoo/vmux-c98tabbar'
```

Leave terminal insert mode by typing escap twice: 
```vimscript
tnoremap <Esc><Esc> <C-\><C-n>
```

# architecture

## crates it relies on

This project relies on the following fundamentals crates:

- wallpapers are displayed via [blockish](https://github.com/yazgoo/blockish/)
- fuzzy prompting is done via [skim](https://github.com/lotabout/skim/)
- terminal session management: [diss](https://github.com/yazgoo/diss)
- historized ordering of selections (BAcked Up Sorter) : [baus](https://github.com/yazgoo/baus) 
