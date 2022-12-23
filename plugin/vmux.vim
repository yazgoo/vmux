function! VmuxDoneEditingCallback()
  let l:s = system("touch " . g:vmux_done_file_path)
endfunction

function! VmuxAddDoneEditingCallback()
  let l:winid = bufwinid(g:vmux_edited_file_path)
  execute("autocmd WinClosed ". l:winid . " ++once call VmuxDoneEditingCallback()")
endfunction

function! VmuxSplit()
  split +terminal | normal i 
endfunction
command! VmuxSplit :call VmuxSplit()

function! VmuxVsplit()
  vsplit +terminal | normal i 
endfunction
command! VmuxVsplit :call VmuxVsplit()

function! VmuxTabnew()
  tabnew +terminal | normal i 
endfunction
command! VmuxTabnew :call VmuxTabnew()

function! VmuxDetachCallback()
  wshada
endfunction

function! VmuxAttachCallback()
  rshada
endfunction
