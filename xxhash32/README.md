## xxhash32 utility

This is a utility executable for hashing things using the custom xxhash32-lib hasher.

```
Usage: xxhash32.exe <COMMAND>

Commands:
  hash         Hashes a single string
  hash-file    Hashes each line of a file and writes the results to a CSV output file
  brute-force  Brute forces a hash, trying to find a string up to a given length that hashes to the given hash
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
