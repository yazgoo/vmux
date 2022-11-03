#/bin/sh
# setup neovim
apt -y update
apt -y install neovim


# install vim-plug 
sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'

# add vim config and plugin
mkdir -p ~/.config/nvim
cat > ~/.config/nvim/init.vim <<EOF
call plug#begin()
Plug 'yazgoo/vmux', {'do': 'cargo install vmux' }
call plug#end()
EOF

# install plugin
nvim +PlugInstall +qa

# create dev directories
mkdir -p ~/dev/project_0 ~/dev/project_1

# create hooks
mkdir -p ~/.config/vmux/hooks

# hook to list new sessions by listing files in ~/dev
cat > ~/.config/vmux/hooks/list_sessions_names.sh <<EOF
#!/bin/sh
find -L ~/dev -maxdepth 1 -type d | while read d; do basename \$d; done
EOF

# on new session hook: change directory
cat > ~/.config/vmux/hooks/session_name.sh <<EOF
#!/bin/sh
mypwd=\$HOME/dev/"\$1"
if [ -e "\$mypwd" ]
then
echo PWD=\$mypwd
fi
EOF

chmod +x ~/.config/vmux/hooks/*

mkdir ~/.config/vmux/wallpapers
cp ~/.local/share/nvim/plugged/vmux/vmux.png ~/.config/vmux/wallpapers
