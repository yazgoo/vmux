<img src="vmux.png" alt="vmux logo" width="200"/>

Helper to use vim/neovim as a terminal multiplexer

# demos

video of session switching:

[![Demo](https://img.youtube.com/vi/TIZZL5dFtQc/0.jpg)](https://www.youtube.com/watch?v=TIZZL5dFtQc)

video (CLI + functionalities inside vim):

[![Demo](https://img.youtube.com/vi/CnLlT0Wd_wY/0.jpg)](https://www.youtube.com/watch?v=CnLlT0Wd_wY)

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

Run `vmux` for vmux command usage help
Run `:help vmux` from within vim for more in depth help.

# detaching

You can detach from the session with ^g

After detaching / or quitting vim, you will be prompted to:

- switch session
- create a new session
- exit

# customizing

## session name

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
