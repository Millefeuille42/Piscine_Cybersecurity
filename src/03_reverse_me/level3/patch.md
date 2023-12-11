Open binary with r2
- `aaaa` -> run analysis
- `s main` -> go to main
- `pdf` -> print main disas

Same as previous exercise, we want to change the jump that occurs after the `scanf`
However, there is no `ok` function. We can see weird calls to inside a long if-else branching:
`___syscall_malloc()` and `____syscall_malloc()`.

- `s sym.____syscall_malloc()` -> go to the function code
- `pdf` -> print disas of the function

We can see a `puts` call and then a return. The `puts` call uses a string corresponding to "Good job.".
From this point we know that this function is the one we need.

After the call to this function, there is a jump instruction that points to the end of the program.

- `oo+` -> reopen the file as read-write
- `s 0x0000135a` -> go to the address where the `je` is
- `wa jmp 0x0000155e` -> change the instruction at the current address to jump to the ok part of the code
- `pd 1` -> check that the change occured
- `q` -> quit r2

Now the binary is patched and ready to accept any password!
