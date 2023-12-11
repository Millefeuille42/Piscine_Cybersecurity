Open binary with r2
- `aaaa` -> run analysis
- `s main` -> go to main
- `pdf` -> print main disas

You see the following function calls

- `0x00001227` -> calls `scanf` to get user input
- `0x0000123c` -> calls `strcmp` to compare user input to password

After which there is a `cmp eax, 0` which compares the return value of `strcmp` to the address where the password is stored
Then, the result of this comparison is used by a `jne` op. It is easy to guess that if `jne` is true, it will jump to the
section that handles a wrong password.

Simplest way to override this jump is to change the `jne` to a `nop`

- `oo+` -> reopen the file as read-write
- `s 0x00001244` -> go to the address where the `jne` is
- `wao nop` -> change the instruction at the current address to `nop`
- `pd 1` -> check that the change occured
- `q` -> quit r2

Now the binary is patched and ready to accept any password!
