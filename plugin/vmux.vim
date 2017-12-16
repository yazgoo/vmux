function! VmuxDoneEditing()
  execute(":x")
  let l:s = system("touch " . g:vmux_done_file_path)
endfunction
command! VmuxDoneEditing call VmuxDoneEditing()
