FROM rust:slim

# setup neovim
RUN apt -y update
RUN apt -y install neovim curl git

# install vim-plug 
RUN sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'

# create neovim custom configuration
RUN mkdir -p /root/.config/nvim/
COPY init.vim /root/.config/nvim/

# install vmux plugin
RUN nvim --headless +PlugInstall +qa

# setup hooks
RUN mkdir -p /root/.config/vmux/hooks/
COPY list_sessions_names.sh /root/.config/vmux/hooks/
COPY session_name.sh /root/.config/vmux/hooks/

# integrate vmux with bash
RUN echo 'source ~/.local/share/nvim/plugged/vmux/plugin/setup_vmux.sh' >> ~/.bashrc

# add vmux logo in wallpapers directory
RUN mkdir ~/.config/vmux/wallpapers
RUN cp ~/.local/share/nvim/plugged/vmux/vmux.png ~/.config/vmux/wallpapers

RUN rm -rf ~/.cargo
RUN apt clean -y
RUN apt remove -y curl git
# create example dev directories
RUN mkdir -p ~/dev/first_project ~/dev/second_project

# start vmux with container
ENTRYPOINT ["/bin/bash", "-c", "sleep 0.1 && vmux new"]
