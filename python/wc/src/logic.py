import os


def build_output(args):
    file_path = args.FILE_PATH
    do_print_bytes = args.c
    do_print_lines = args.l

    # TODO:
    # do_print_words = args.w

    # TODO:
    # do_print_character_count = args.m

    # TODO: support default options - {lines} | {words} | {bytes} | {file_path?}
    # TODO: read part of a file into a buffer, then dump the memory before reading the next bit

    output = ""

    # TODO:
    if file_path is None:
        print("todo: read from stdin")
        return

    if do_print_bytes:
        file_size = os.path.getsize(file_path)
        output += str(file_size)

    if do_print_lines:
        print("todo: count the words")

    if file_path is not None:
        output += f" {file_path}"

    return output
