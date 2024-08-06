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

    const pushToOutput = (val: string) => {
      addSpacingToOutput ? (output += ` ${val}`) : (output += val);
    };

    const countLines = () => {
      const lineCount = data.split(/\r\n|\r|\n/).length - 1;

      if (addSpacingToOutput) {
        // weird behavior from wc
        pushToOutput(` ${lineCount.toString()} `);
      } else {
        pushToOutput(lineCount.toString());
      }
    };

    const countWords = () => {
      const words = data.split(/\s+/);
      let wordCount = words.length;

      if (wordCount > 0) wordCount--;

      pushToOutput(wordCount.toString());
    };

    const countCharacters = () => {
      const characterCount = data.split("").length;
      pushToOutput(characterCount.toString());
    };

    const countBytes = () => {
      const byteCount = Buffer.byteLength(data);
      pushToOutput(byteCount.toString());
    };

    // TODO: read in chunks rather than entire file
    const data = filePath
      ? (readFile(filePath) as string)
      : ((await new Promise((resolve) => {
          process.stdin.resume();
          process.stdin.setEncoding("utf8");
          process.stdin.on("data", (text) => resolve(String(text)));
        })) as string);

    process.stdin.pause();

    if (lines) countLines();
    if (words) countWords();
    if (characters) countCharacters();
    if (bytes) countBytes();
    if (useDefaultOptions) {
      countLines();
      countWords();
      countBytes();
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

function readFile(filePath: string) {
  try {
    return readFileSync(filePath).toString();
  } catch (err) {
    if (err instanceof Error && "code" in err) {
      if (err.code === "ENOENT") throw `File not found: ${filePath}`;
    } else {
      throw err;
    }
  }
}
