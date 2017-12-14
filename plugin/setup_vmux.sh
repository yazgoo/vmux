__is_running_in_nvim() {
  (ps -o comm= $PPID | grep nvim > /dev/null)
}
if ! __is_running_in_nvim
then
  vmux() {
    session_prefix="nvim-session-"
    if [ $# -gt 0 ] && [ "$1" = list ]
    then
      abduco -l | grep "$session_prefix"
    elif [ $# -gt 1 ] && [ "$1" = attach ]
    then
      abduco -e '^g' -A "$2"
    elif [ $# -gt 0 ] && [ "$1" = new ]
    then
      id=$(date +%s)
      server_file="/tmp/vim-server-$id"
      session_name="$session_prefix$id"
      vmux_server_file=$server_file abduco -e '^g' -A "$session_name" nvim \
        --cmd "let g:confirm_quit_nomap = 0"\
        --cmd "let g:server_addr = serverstart('$server_file')"
    else
      echo "please provide an action (new|attach|list)"
    fi
  }
  return
fi
vmux_plugin_directory=$(dirname "$0")
export PATH="$PATH:$vmux_plugin_directory"
export EDITOR=editor


cd() {  
  builtin cd "$@";
  vmux_send :cd "$PWD"
  echo "$PWD" > ~/.cwd;
}
