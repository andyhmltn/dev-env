
-- Create a react component tag from word
vim.keymap.set('n', '<Leader>-crt', function() 
	vim.cmd('normal ysiw>')
	vim.cmd('normal ea/')
	vim.cmd('normal ga')
end, { noremap = false })
 
--  Open react tag <Example></Example>
vim.keymap.set('n', '<Leader>-ort', function() 
	vim.cmd('normal yiwysiw>A')
	vim.cmd('normal pysiw>a/')
	vim.cmd('normal hi')
end, { noremap = false })
 
--  Use state const [ example , setExample] = useState()
vim.keymap.set('n', '<Leader>-ust', function() 
	vim.cmd('normal yiwysiw[Iconst ')
	vim.cmd('normal f]i, ')
	vim.cmd('normal pbvUiset')
	vim.cmd('normal A = useState(')
	vim.api.nvim_feedkeys('a', 'n', true)
end, { noremap = false })


-- Paste param (eg: xyz in clipboard becomes xyz={xyz})
vim.keymap.set('n', '<Leader>ypp', function() 
	vim.cmd('normal a ')
	vim.cmd('normal pa=')
	vim.cmd('normal pysiw{:w<Cr>f ')
end, { noremap = false })

-- Copy the current filename to the clipboard, formatted as a component name
vim.keymap.set('n', '<Leader>-nn', function() 
	vim.cmd('put %')
	vim.cmd('normal $F.yT/ddpV:CamelB<Cr>')
	vim.cmd('normal _vUviwx')
end, { noremap = false })

