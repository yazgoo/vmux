#!/bin/env sh
yq -r <$(dirname $(dirname $(dirname $0)))/presentation.yml 'keys_unsorted[]' | ruby <(echo 'STDIN.each_line.to_a.each_with_index { |l, i| puts "0x#{i.to_s(16)}) #{l}" }')| tac
