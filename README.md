# vmux

![vmux logo](vmux.png)

Helper to use neovim as a terminal multiplexer

Demo video:

[![Demo](https://img.youtube.com/vi/CnLlT0Wd_wY/0.jpg)](https://www.youtube.com/watch?v=CnLlT0Wd_wY)

# install

First, you need to install [abduco](https://github.com/martanne/abduco).

Add and install the following vim plugin: 

```
Plug 'yazgoo/vmux'
```

Then add the following to your .zshrc or .bashrc

```
source ~/.config/nvim/plugged/vmux/plugin/setup_vmux.sh
```

Or if you want to use vim instead of nvim (you need vim compiled with `+clientserver` flag) :

```
source ~/.config/nvim/plugged/vmux/plugin/setup_vmux.sh vim
```

# usage

Run `vmux` for vmux command usage help
Run `:help vmux` from within vim for more in depth help.

# detaching

You can detach from the session with ^g

# customizing

Having a nice tabbar (based on [Caagr98/c98tabbar.vim](https://github.com/Caagr98/c98tabbar.vim)):

```
Plug 'git@github.com:yazgoo/c98tabbar.vim'
Plug 'yazgoo/vmux-c98tabbar'
```

Leave terminal insert mode by typing `jj`: 
```
tnoremap jj <C-\><C-n>
```
