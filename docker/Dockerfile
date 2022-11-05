FROM rust
COPY setup-alpine.sh /
RUN mkdir -p /root/.config/nvim/
COPY init.vim /root/.config/nvim/
RUN mkdir -p /root/.config/vmux/hooks/
COPY list_sessions_names.sh /root/.config/vmux/hooks/
COPY session_name.sh /root/.config/vmux/hooks/
RUN /setup-alpine.sh
RUN echo "sleep 0.2 ; vmux new" > ~/.bashrc
