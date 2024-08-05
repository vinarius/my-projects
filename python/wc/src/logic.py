import os


def build_output(args):
    file_path = args.FILE_PATH
    do_print_bytes = args.c
    do_print_lines = args.l
    do_print_words = args.w
    do_print_character_count = args.m

    def process_large_file(operation):
        bytes = 1024
        kilobytes = 1024
        count = 0

        with open(file_path, "rb") as file:
            while True:
                chunk = file.read(bytes * kilobytes)

                if not chunk:
                    break

                count += operation(chunk)

        return count

    # TODO: support default options - {lines} | {words} | {bytes} | {file_path?}
    # TODO: read part of a file into a buffer, then dump the memory before reading the next bit
    # TODO: support multiple options output formatting

    output = ""

    # TODO:
    if file_path is None:
        print("todo: read from stdin")
        return

    if do_print_bytes:
        file_size = os.path.getsize(file_path)
        output += str(file_size)

    if do_print_lines:
        line_count = process_large_file(count_lines)
        output += str(line_count)

    if do_print_words:
        word_count = process_large_file(count_words)
        output += str(word_count)

    if do_print_character_count:
        # character_count = process_large_file(count_characters)

        with open(file_path, "r") as file:
            content = file.read()
            character_count = 0

            for line in content.splitlines():
                character_count += len(line)

        output += str(character_count)

    if file_path is not None:
        output += f" {file_path}"

    return output


def count_lines(chunk):
    return chunk.count(b"\n")


def count_words(chunk):
    return len(chunk.split())


def count_characters(chunk):
    return len(
        chunk.replace(b"\n", b"")
    )  # doesn't produce the correct result, can't figure this out
