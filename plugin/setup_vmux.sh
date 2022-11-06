if [ -z "$BASH_SOURCE" ]
then
  export PATH="$PATH:$(dirname "$0")"
else
  export PATH="$PATH:$(dirname "$BASH_SOURCE")"
fi

first_arg_possibilities="new list attach"

function list_vmux_sessions() {
  vmux list |sed 's/\t/|/g' |cut -d\| -f3
}

if [ -n "$ZSH_VERSION" ]
then
  compdef _vmux vmux  

  function _vmux {
    local _line
    _arguments -C "1: :($first_arg_possibilities)" "*::arg:->args"

    case $line[1] in
      attach) _vmux_attach
    esac

  }

  function _vmux_attach {
    _arguments -C "1: :($(list_vmux_sessions))" "*::arg:->args"
  }
elif [ -n "$BASH_VERSION" ]
then
  function _vmux {
     local cur prev

    cur=${COMP_WORDS[COMP_CWORD]}
    prev=${COMP_WORDS[COMP_CWORD-1]}

    case ${COMP_CWORD} in
        1)
            COMPREPLY=($(compgen -W "$first_arg_possibilities" -- ${cur}))
            ;;
        2)
            case ${prev} in
                attach)
                  COMPREPLY=($(compgen -W "$(list_vmux_sessions)" -- ${cur}))
                    ;;
            esac
            ;;
        *)
            COMPREPLY=()
            ;;
    esac
  }
complete -F _vmux vmux
fi
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
