Open binary with r2
- `aaaa` -> run analysis
- `s main` -> go to main
- `pdf` -> print main disas

You see the following function call

- `0x0000130e` -> calls `scanf` to get user input

After which there is a `cmp eax, 1` which compares the return value of `scanf` to 1, this means "scanf has read 1 item"
Then, the result of this comparison is used by a `je` op. It is easy to guess that if `je` is false, it will continue to the
section that handles a wrong password, and else it will jump to the rest of the program.

However, unlike the previous level, there is way more stuff and checks happening after the jump,
we could `nop` everything, including the calls to `no` but it could be tedious and break stuff at some point.

Simplest way to override the password check is to change this conditional jump to a later moment in code
where the "ok" route is handled.

Turns out we can see this "ok route" later in the code:

- `0x00001476` -> call to internal `ok` function, we should check what this does
- `0x0000147b` -> jump to the cleanup and return part of the code

Running: 
- `s sym.ok`-> got to the code of the `ok` function
- `pdf` -> print `ok` disas

We can see that this functions `puts` a string and the returns. Due to it's name and behavior we might assume
that this is what we are looking for 

- `oo+` -> reopen the file as read-write
- `s 0x0000131e` -> go to the address where the `je` is
- `wa jmp 0x1476` -> change the instruction at the current address to jump to the ok part of the code
- `pd 1` -> check that the change occured
- `q` -> quit r2

Now the binary is patched and ready to accept any password!
