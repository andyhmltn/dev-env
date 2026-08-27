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

-- Insert a className on the current tag
vim.keymap.set('n', '<Leader>-cn', function()
	vim.cmd('normal _f>i className="')
	vim.cmd('normal l')
	vim.cmd('startinsert')
end, { noremap = false })

-- Copy the current filename to the clipboard, formatted as a component name
vim.keymap.set('n', '<Leader>-nn', function()
	vim.cmd('put %')
	vim.cmd('normal $F.yT/ddpV:CamelB<Cr>')
	vim.cmd('normal _vUviwx')
end, { noremap = false })

-- test?: hello
-- Delete props interface
vim.keymap.set('n', '<Leader>dpi', '/interface <Cr>2dd/:<Cr>dt)di(', { noremap = false })

-- Change required prop to optional
vim.keymap.set('x', '<Leader>::', ':s/:/?:/<Cr>:nohlsearch<Cr><Cr>', { noremap = false })

-- Change optional prop to required
vim.keymap.set('x', '<Leader>??', ':s/?//g<Cr>:nohlsearch<Cr><Cr>', { noremap = false })

-- console.log()
vim.keymap.set('n', '<Leader>-cl',function()
	vim.cmd("normal iconsole.log()")
	vim.cmd("normal h")
end, { noremap = false })
-- Take the current clipboard variable (eg: example) and console log it with a label (eg: console.log("example", example))
vim.keymap.set('n', '<Leader>-ca',function()
	vim.cmd("normal iconsole.log()")
	vim.cmd("normal hpysiw'$i, ")
	vim.cmd("normal p")
end, { noremap = false })
-- JSON.stringify current word
vim.keymap.set('n', '<Leader>-js',function()
	vim.cmd("normal ysiw)iJSON.stringify")
end, { noremap = false })

-- JSON.stringify
vim.keymap.set('n', '<Leader>-aj', 'iJSON.stringify', { noremap = false })

-- Change variable to currentVariable
vim.keymap.set('n', '<Leader>-cv', 'vUicurrent', { noremap = false })

-- throw new Error
vim.keymap.set('n', '<Leader>-tn',function()
	vim.cmd("normal ithrow new Error(")
end, { noremap = false })
-- Create a useMemo
vim.keymap.set('n', '<Leader>-um',function()
	vim.cmd("normal ys$(a(")
	vim.cmd("normal la=>")
	vim.cmd("normal IuseMemo")
	vim.cmd("normal A")
	vim.cmd("normal i,[h")
end, { noremap = false })


-- Convert current word to exported interface
vim.keymap.set('n', '<Leader>-i',function()
	vim.cmd("normal viwyIinterface ")
	vim.cmd("normal A {}")
end, { noremap = false })
