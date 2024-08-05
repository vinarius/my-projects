from parse_args import parse_args
from logic import build_output


def main():
    args = parse_args()
    output = build_output(args)

    if output:
        print(output)


if __name__ == "__main__":
    main()
