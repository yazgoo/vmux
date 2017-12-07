vmux() {
  if [ $# -gt 1 ] && [ "$1" = attach ]
  then
    abduco -e '^g' -A "$2"
  else
    id=$(date +%s)
    server_file="/tmp/vim-server-$id"
    session_name=nvim-session-$id
    vmux_server_file=$server_file abduco -e '^g' -A "$session_name" nvim \
      --cmd "let g:confirm_quit_nomap = 0"\
      --cmd "let g:server_addr = serverstart('$server_file')"
  fi
}

alias vmux_send=~/.config/nvim/send_command_to_vim_session.py

for cmd in split vsplit e tabnew
do
  alias $cmd="vmux_send :$cmd"
done

cd() {  
  builtin cd "$@";
  (ps -o comm= $PPID | grep nvim > /dev/null) && vmux_send :cd "$@"
  echo "$PWD" > ~/.cwd;
}

