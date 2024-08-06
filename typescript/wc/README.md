# wc - word, line, character, and byte count. 

### Run with --help to print usage
```bash
> npx ts-node src/main.ts --help
```

Sample file found in data/test.txt

Usage: main [options] [FILE_PATH]

Options:
  - -c, --bytes       Output the number of bytes in a file (default: false)
  - -l, --lines       Output the number of lines in a file (default: false)
  - -w, --words       Output the number of words in a file (default: false)
  - -m, --characters  Output the number of characters in a file (default: false)
  - -h, --help        display help for command

```bash
> npx ts-node src/main.ts -c data/test.txt
335041 data/test.txt
```

If no file is specified, will read from stdin.

```bash
> echo "foo" | npx ts-node src/main.ts
  1  1 4
```

