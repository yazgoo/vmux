#!/bin/env sh
presentation_root=$(dirname $(dirname $(dirname $0)))
tmp_init_vim="/tmp/tmp.neovimconf.init.vim"
args="-u $tmp_init_vim "
get_page() {
  set -x
  yq -r <"$presentation_root"/presentation.yml ".[\"$(echo $1 | sed 's/^[^ ]\+ //')\"].$2" 
}

split=$(get_page "$1" split)
if [ "$split" != null ]
then
  args="$args -$split"
fi

img=$(get_page "$1" img)
if [ "$img" != null ]
then
  echo IMG_PATH="$presentation_root/img/$img"
  args="$args term://$presentation_root/render_img "
fi

term=$(get_page "$1" term | sed 's;presentation_root;'$presentation_root';')
if [ "$term" != null ]
then
  args="$args term://$term "
fi


render_file=/tmp/$(date +%s).md
(
echo $1
echo $1 | sed 's/./=/g'
echo
get_page "$1" text
)> "$render_file"
args=$(echo "$args" | sed 's/ \+/ /g')

echo VMUX_ADDITIONAL_ARGUMENTS="$args$render_file"
