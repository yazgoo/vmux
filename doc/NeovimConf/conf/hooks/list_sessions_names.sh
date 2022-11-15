#!/bin/env sh
yq -r <$(dirname $(dirname $(dirname $0)))/presentation.yml 'keys_unsorted[]' | nl -w2 -s'. ' | tac
