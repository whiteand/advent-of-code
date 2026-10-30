import process from "node:process";
import util from "node:util";

export function parseDownloadArgs() {
  const AOC = process.env["AOC"];
  if (!AOC) {
    throw new Error(
      "AOC variable is not defined." +
        "\n" +
        'Please consult "session" cookie from the logged in state of the advent' +
        "\n" +
        "set AOC=<cookie value> in .env",
    );
  }

  const { positionals } = util.parseArgs({
    allowPositionals: true,
  });

  const yearString = positionals[0];
  const dayString = positionals[1];
  if (!yearString || !/^\d{2}$/.test(yearString)) {
    printHelp();
    process.exit(1);
  }
  if (!dayString || !/^\d{1,2}$/.test(dayString)) {
    printHelp();
    process.exit(1);
  }
  let year = Number.parseInt(yearString, 10);
  if (year < 100) {
    year += 2000;
  }
  const day = Number.parseInt(dayString, 10);

  return {
    AOC,
    year,
    day,
  };
}

function printHelp() {
  console.log(`node ${process.argv.slice(1, 2)} <year> <day>`);
  console.log(`Example`);
  console.log(`  node ${process.argv.slice(1, 2)} 26 2`);
  console.log(`  node ${process.argv.slice(1, 2)} 26 12`);
}
