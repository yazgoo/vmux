#/bin/sh
# setup neovim
apt -y update
apt -y install neovim


# install vim-plug 
sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'

# install plugin
nvim --headless +PlugInstall +qa

# create dev directories
mkdir -p ~/dev/first_project ~/dev/second_project

chmod +x ~/.config/vmux/hooks/*

mkdir ~/.config/vmux/wallpapers
cp ~/.local/share/nvim/plugged/vmux/vmux.png ~/.config/vmux/wallpapers
