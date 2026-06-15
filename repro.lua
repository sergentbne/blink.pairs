vim.pack.add({
  'https://github.com/saghen/blink.download',
  { src = 'https://github.com/saghen/blink.pairs', version = vim.version.range('*') },
})
local blink = require('blink.pairs')
blink.build():pwait(60000)
blink.setup({})
