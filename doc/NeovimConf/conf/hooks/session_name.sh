#!/bin/env sh
render_file=/tmp/$(date +%s)
yq -r <$(dirname $(dirname $(dirname $0)))/presentation.yml ".[\"$(echo $1 | sed 's/^.*\. //')\"]" > "$render_file"
echo VMUX_ADDITIONAL_ARGUMENTS="$render_file"
