let unittest_dispatch_array =<< END
blabla
END
let unittest_dispatch_unformatted = join(unittest_dispatch_array)
let @t = "cargo run"

noremap <leader>sr :tabprevious<CR>:execute "Start" @r<CR><CR>
noremap <leader>st :tabprevious<CR>:execute "Start " @t<CR><CR>

" autocmd BufWritePost dispatch.vim source dispatch.vim
