export PATH="$PATH:$(dirname "$0")"
if [ $# -gt 0 ]
then
  VMUX_EDITOR="$1"
else
  VMUX_EDITOR="nvim"
fi
export VMUX_EDITOR
[ -z "$vmux_server_file" ] && return
# specific stuff to vmux session
export EDITOR=vmux-editor
cd() {  
  builtin cd "$@";
  vmux_send :tcd "$PWD"
  echo "$PWD" > ~/.cwd;
}
