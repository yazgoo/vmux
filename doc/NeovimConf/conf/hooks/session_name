#!/bin/env sh
export _hash=$(echo "$1" | md5sum | cut -d\  -f1)
export args_file="/tmp/tmp.neovimconf.$_hash.args"
echo "$args_file" > /tmp/lol
cat $args_file
