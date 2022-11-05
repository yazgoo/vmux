#!/bin/sh
find -L ~/dev -maxdepth 1 -type d | while read d; do basename $d; done
