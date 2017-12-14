vmux_plugin_directory=$(dirname "$0")
export PATH="$PATH:$vmux_plugin_directory"
export EDITOR=editor

cd() {  
  builtin cd "$@";
  vmux_send :cd "$PWD"
  echo "$PWD" > ~/.cwd;
}
