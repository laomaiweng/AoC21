let b:current_syntax = "cel"

syn keyword aluInstruction inp add mul div mod eql
syn match aluLiteral '-\?\d\+'
syn match aluLabel '^.\+:$'
syn match aluComment '#.\*:$'

hi def link aluComment     Comment
hi def link aluInstruction Statement
hi def link aluLiteral     Constant
hi def link aluLabel       PreProc
