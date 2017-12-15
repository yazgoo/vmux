export PATH="$PATH:$(dirname "$0")"
[ -z "$vmux_server_file" ] && return
# specific stuff to vmux session
export EDITOR=editor
cd() {  
  builtin cd "$@";
  vmux_send :cd "$PWD"
  echo "$PWD" > ~/.cwd;
}
