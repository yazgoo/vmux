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
export EDITOR=editor
cd() {  
  builtin cd "$@";
  vmux_send :cd "$PWD"
  echo "$PWD" > ~/.cwd;
}
