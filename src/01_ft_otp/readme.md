# 01 - FT_OTP

## Goal

In the language of your choice, you have to implement a program that allows you to store
an initial password in file, and that is capable of generating a new one time password
every time it is requested.

You can use any library that facilitates the implementation of the algorithm, as long
as it doesn’t do the dirty work, i.e. using a TOTP library is strictly prohibited. Of
course, you can and should use a library or function that allows you to access system
time.

- The executable must be named ft_otp
- Your program must take arguments.
  - g: The program receives as argument a hexadecimal key of at least 64 characters. 
The program stores this key safely in a file called ft_otp.key, which
is encrypted.
  - k: The program generates a new temporary password based on the key given
as argument and prints it on the standard output.
- Your program must use the HOTP algorithm (RFC 4226).
- The generated one-time password must be random and must always contain the
same format, i.e. 6 digits.

Below is an example of use:

```bash
$ echo -n "NEVER GONNA GIVE YOU UP" > key.txt
$ ./ft_otp -g key.txt
./ft_otp: error: key must be 64 hexadecimal characters.
$ [..]
$ cat key.hex | wc -c
64
$ ./ft_otp -g key.hex
Key was successfully saved in ft_otp.key.
$ ./ft_otp -k ft_otp.key
836492
$ sleep 60
$ ./ft_otp -k ft_otp.key
123518
```

You can check if your program is working properly by comparing generated passwords
with Oathtool or any tool of your choice.

`oathtool –totp $(cat key.hex)`