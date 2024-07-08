# wc - word, line, character, and byte count. 

### Run with --help to print usage
```bash
> python3 src/main.py --help
```

Sample file found in data/test.txt

Options:
- -c: outputs the number of bytes in a file
- -l: outputs the number of lines in a file
- -w: outputs the number of words in a file
- -m: outputs the number of characters in a file

```bash
> python3 src/main.py -c test.txt
342190 test.txt
```

If no file is specified, will read from stdin.

ie
```bash
> echo "foo" | python3 src/main.py
1 1 3
```
