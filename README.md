<img src="vmux.png" alt="vmux logo" width="200"/>

Helper to use vim/neovim as a terminal multiplexer

Demo video:

[![Demo](https://img.youtube.com/vi/CnLlT0Wd_wY/0.jpg)](https://www.youtube.com/watch?v=CnLlT0Wd_wY)

# install

First, you need to install [abduco](https://github.com/martanne/abduco).

Add and install the following vim plugin: 

```vimscript
Plug 'yazgoo/vmux'
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
After detaching / or quitting vim, you will be prompted, via fzf to:
- switch session
- create a new session
- exit

# customizing

## session name


You can define a custom way to declare a new session via `~/.config/vmux/hooks/session_name.sh`
The script just needs to export `session_name`.

For example, this script will select the session name from the directory names in `~/dev/`, and change directory:

```
export session_name=$(ls ~/dev | fzf)
cd ~/dev/$session_name
```

## tabbar

Having a nice tabbar (based on [Caagr98/c98tabbar.vim](https://github.com/Caagr98/c98tabbar.vim)):

```vimscript
Plug 'git@github.com:yazgoo/c98tabbar.vim'
Plug 'yazgoo/vmux-c98tabbar'
```

Leave terminal insert mode by typing escap twice: 
```vimscript
tnoremap <Esc><Esc> <C-\><C-n>
```
