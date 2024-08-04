from argparse import ArgumentParser


def parse_args():
    parser = ArgumentParser()

    parser.add_argument(
        "-c",
        action="store_true",
        default=False,
        help="outputs the number of bytes in a file",
    )

    parser.add_argument(
        "-l",
        action="store_true",
        default=False,
        help="outputs the number of lines in a file",
    )

    parser.add_argument(
        "-w",
        action="store_true",
        default=False,
        help="outputs the number of words in a file",
    )

    parser.add_argument(
        "-m",
        action="store_true",
        default=False,
        help="outputs the number of characters in a file",
    )

    parser.add_argument(
        "FILE_PATH",
        # nargs makes positional arguments optional
        nargs="?",
    )

    args = parser.parse_args()

    return args
