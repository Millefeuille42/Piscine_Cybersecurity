# Stockholm

Ciphers the target folder as defined by the subject (`~/infection`).
Can also reverse the operation with the -r flag.

## Usage

Running Stockholm without any parameters will cipher the target folder and write
the base64-encoded key in `./stockholm.key`.

Running it with the `-r / --reverse` flag will decipher the target folder. This flag reads
the base-64 encoded key from the argument provided for the flag.

Using the `-s / --silent` flag will silence the program, even errors won't be logged.

```
Usage: stockholm [OPTIONS]

Options:
  -v, --version        prints the current version
  -s, --silent         do not print anything
  -r, --reverse <KEY FILE>  decrypts the target folder
  -h, --help           Print help

```

## Start
To use the program, compile it with `make` and generate the sample data with `make populate`.
The target folder will be populated (warning, any file that has the same name as the sample data will be overwritten)

Then run Stockholm with `./stockholm`. To decipher the target folder, run Stockholm with `./stockholm -r ./stockholm.key`

## Note
This program will only cipher a definite list of extensions. This list can be found
in the `src/constants.rs` source file at line 4.