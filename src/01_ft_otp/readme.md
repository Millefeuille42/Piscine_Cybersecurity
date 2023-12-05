# ft_otp
Implementation of the TOTP algorithm. 
- Generates a key file from a 64 characters hexadecimal key provided in argument
- Generates a TOTP with a 30 seconds rotation from a key file provided in argument

```
Usage: ft_otp -g <KEY_FILE> -k <KEY_FILE>

Options:
  -g <KEY_FILE>      Generate key file based on a hexadecimal key
  -k <KEY_FILE>      Get TOTP based on provided key file
  -h, --help         Print help
```