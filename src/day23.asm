01      cpy a b
02      dec b           ; b=a-1

    loop6:
03      cpy a d         | for(d=a; d!=0; d--) ...
04      cpy 0 a         ; a=0
    loop2:              |
05      cpy b c         | | for(c=b; c!=0; c--) ...
    loop1:              | |
06      inc a           | | a++
07      dec c           | |
08      jnz c loop1     | |
09      dec d           |
10      jnz d loop2     |

11      dec b           ; b-=1
12      cpy b c         ; c=b

13      cpy c d         | for(d=c; d!=0; d--) ...
    loop3:              |
14      dec d           |
15      inc c           | c++
16      jnz d loop3     |

17      tgl c           |

18      cpy -16 c
19      jnz 1 c         ; jmp loop6                 -> toggled to: cpy 1 c

20      cpy 84 c

    loop5:
21      jnz 75 d        | jmp [d]                   -> toggled to: cpy 75 d
                        |
    loop4:              | | for(d=?; d!=0; d++) ...
22      inc a           | | a++
23      inc d           | |                         -> toggled to: dec d
24      jnz d loop4     | |
                        |
25      inc c           |                           -> toggled to: dec c
26      jnz c loop5     |
