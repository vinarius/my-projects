import os
from argparse import ArgumentParser

def main():
    args = parse_args()

    # print('args', args)

    file_path = args.FILE_PATH

    try:
        file_size = os.path.getsize(file_path)
    except OSError as err:
        print(err)
        return

    print(f'{file_size} {file_path}')



def parse_args():
    parser = ArgumentParser()

    parser.add_argument(
        '-c',
        action='store_true',
        default=False,
        help='outputs the number of bytes in a file'
    )

    parser.add_argument(
        '-l',
        action='store_true',
        default=False,
        help='outputs the number of lines in a file'
    )

    parser.add_argument(
        '-w',
        action='store_true',
        default=False,
        help='outputs the number of words in a file'
    )

    parser.add_argument(
        '-m',
        action='store_true',
        default=False,
        help='outputs the number of characters in a file'
    )

    parser.add_argument(
        'FILE_PATH',
    )

    args = parser.parse_args()

    return args

if __name__ == '__main__':
    main()
