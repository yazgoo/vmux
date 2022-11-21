call plug#begin()
Plug 'yazgoo/vmux', {'do': 'cargo install vmux' }
call plug#end()


command! VD :VmuxDoneEditing
" terminal splits shortcuts
nnoremap <space>s :VmuxSplit<cr>
nnoremap <space>v :VmuxVsplit<cr>
nnoremap <space>h :winc h<cr>
nnoremap <space>j :winc j<cr>
nnoremap <space>k :winc k<cr>
nnoremap <space>l :winc l<cr>
nnoremap <space>q :qa!<cr>

nnoremap <space>w :w<cr>
nnoremap <space>t :tabnew<cr>
" double escape to leave terminal
tnoremap <Esc><Esc> <C-\><C-n>

" custom status line for 
set laststatus=2
set statusline=
set statusline+=%F
set statusline+=%=
set statusline+=%{substitute(getcwd(),'^.*/','','')}

set shell=/bin/bash
set termguicolors
hi! link StatusLine Normal
hi! link StatusLineNC Normal
set statusline=%{repeat(' ',winwidth('.'))}
set fillchars+=eob: 
set fillchars+=vert: 
set laststatus=0

