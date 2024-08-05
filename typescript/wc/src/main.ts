import { program, Option, Argument } from "commander";

function main() {
  program
    .addOption(
      new Option("-c, --bytes", "Output the number of bytes in a file").default(
        false,
      ),
    )
    .addOption(
      new Option("-l, --lines", "Output the number of lines in a file").default(
        false,
      ),
    )
    .addOption(
      new Option("-w, --words", "Output the number of words in a file").default(
        false,
      ),
    )
    .addOption(
      new Option(
        "-m, --characters",
        "Output the number of characters in a file",
      ).default(false),
    )
    .addArgument(new Argument("[FILE_PATH]"))
    .addHelpText("beforeAll", "wc - word, line, characer, and byte count\n");

  program.parse();

  console.log(program.opts());
}

if (require.main === module) main();
