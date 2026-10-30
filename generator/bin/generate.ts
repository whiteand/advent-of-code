import process from "node:process";
import util from "node:util";
import path from "node:path";
import fs from "node:fs/promises";
import child_process from "node:child_process";
import { parseDownloadArgs } from "../lib/parseDownloadArgs.ts";
import { downloadInput } from "../lib/downloadInput.ts";
import { addMember } from "../lib/addMember.ts";

const pwd = process.cwd();

const { AOC, year, day } = parseDownloadArgs();
const inputBody = await downloadInput({ AOC, year, day });

const formatAsDoubleDigit = (n: number) =>
  `${n % 100}`.toString().padStart(2, "0");
const formattedYear = formatAsDoubleDigit(year);
const formattedDay = formatAsDoubleDigit(day);
const folderPath = path.resolve(pwd, `y${formattedYear}/d${formattedDay}`);

const res = await new Promise<string>((resolve, reject) => {
  child_process.exec(
    "/Users/whiteand/.cargo/bin/cargo" +
      " " +
      [
        "generate",
        "--init",
        "--path",
        path.resolve(pwd, "./template"),
        "-d",
        `year=${formattedYear}`,
        "-d",
        `day=${formattedDay}`,
        "--name",
        `y${formattedYear}d${formattedDay}`,
      ].join(" "),
    {
      env: {
        AOC,
        USER: process.env["USER"],
      },
      cwd: pwd,
      signal: AbortSignal.timeout(2000),
    },
    (err, stdout, stderr) => {
      if (err != null) {
        reject(err);
      } else {
        resolve(stdout);
      }
    },
  );
});

// /Users/whiteand/.cargo/bin/cargo generate --init --path ./template -d year={{year}} -d day={{day}} --name y{{year}}d{{day}}

// await await fs.mkdir(folderPath, {
//   recursive: true,
// });

await fs.writeFile(path.resolve(folderPath, "input.txt"), inputBody, "utf-8");
await addMember(
  path.resolve(pwd, "Cargo.toml"),
  `y${formattedYear}/d${formattedDay}`,
);
