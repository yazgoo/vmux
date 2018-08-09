function! VmuxDoneEditing()
  execute(":x")
  let l:s = system("touch " . g:vmux_done_file_path)
endfunction
command! VmuxDoneEditing call VmuxDoneEditing()

function! VmuxSplit()
  split +terminal | normal i 
endfunction

function! VmuxVsplit()
  vsplit +terminal | normal i 
endfunction

function! VmuxTabnew()
  tabnew +terminal | normal i 
endfunction
