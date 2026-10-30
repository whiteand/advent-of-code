import process from "node:process";
import { parseDownloadArgs } from "../lib/parseDownloadArgs.ts";
import { downloadInput } from "../lib/downloadInput.ts";
const { AOC, year, day } = parseDownloadArgs();
const inputBody = await downloadInput({ AOC, year, day });
process.stdout.write(inputBody);
process.stdout.end();
