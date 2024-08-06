import { program, Option, Argument } from "commander";
import { readFileSync } from "fs";

async function main() {
  try {
    program
      .addOption(
        new Option(
          "-c, --bytes",
          "Output the number of bytes in a file",
        ).default(false),
      )
      .addOption(
        new Option(
          "-l, --lines",
          "Output the number of lines in a file",
        ).default(false),
      )
      .addOption(
        new Option(
          "-w, --words",
          "Output the number of words in a file",
        ).default(false),
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

    const options = program.opts();
    const { bytes, lines, words, characters } = options;
    const [filePath] = program.args;
    const useDefaultOptions = Object.values(options).every(
      (opt) => opt === false,
    );
    const isOutputtingAtLeastTwoOptions =
      Object.values(options).reduce((acc, curr) => {
        return curr === true ? acc + 1 : acc;
      }, 0) > 1;
    const addSpacingToOutput =
      isOutputtingAtLeastTwoOptions || useDefaultOptions;
    let output = "";

    if (addSpacingToOutput) output += " ";

    const pushToOutput = (val: string) => {
      addSpacingToOutput ? (output += ` ${val}`) : (output += val);
    };

    // TODO: read in chunks rather than entire file
    const data = filePath
      ? readFileSync(filePath).toString()
      : ((await new Promise((resolve) => {
          process.stdin.resume();
          process.stdin.setEncoding("utf8");
          process.stdin.on("data", (text) => resolve(String(text)));
        })) as string);

    process.stdin.pause();

    if (lines) {
      const lineCount = data.split(/\r\n|\r|\n/).length - 1;
      pushToOutput(lineCount.toString());
    }

    if (bytes) {
      const byteCount = Buffer.byteLength(data);
      pushToOutput(byteCount.toString());
    }

    if (words) {
      console.log("TODO: words");
    }

    if (characters) {
      console.log("TODO: characteres");
    }

    if (filePath) {
      output += ` ${filePath}`;
    }

    console.log(output);
  } catch (err) {
    console.error(err);
    process.exit(1);
  }
}

if (require.main === module) void main();
