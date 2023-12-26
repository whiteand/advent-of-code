import fs from "fs";
import { parse } from "./parse.ts";
import { IModule, IModuleVisitor } from "./modules";
import { solvePart1 } from "./part1.ts";
import { solvePart2 } from "./part2.ts";

const inputType: "input" | "example" = "input";
const FILE_BY_TYPE = {
  input: "./input.txt",
  example: "./example.txt",
};
const EXPECTED_VALUE_1 = {
  input: 938065580,
  example: 32000000,
};

const inputText = fs.readFileSync(FILE_BY_TYPE[inputType], "utf-8").trim();

const modules = parse(inputText);

const PART_1 = 1000;
const solution = solvePart1(modules, PART_1);
console.log("Part 1");
console.log("  Low: ", solution.low);
console.log("  High: ", solution.high);
console.log("  Result: ", solution.low * solution.high);
const EXPECTED = EXPECTED_VALUE_1[inputType];
if (solution.low * solution.high !== EXPECTED) {
  throw new Error("Expected 32000000 but got " + solution);
} else {
  console.log("Correct");
}

const PART_2 = solvePart2(modules);
console.log("Part 2");
console.log("  Button clicked: ", PART_2);
