#!/usr/bin/env python3
import neovim
import sys
import os
nvim = neovim.attach('socket', path=os.environ['vmux_server_file'])
nvim.command(" ".join(sys.argv[1:]))
