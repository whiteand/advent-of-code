import { readFile, writeFile } from "node:fs/promises";
import process from "node:process";
const REGEX = /members = (\[\n(\s+".*?".\n)+?\])/g;

export async function addMember(cargoPath: string, newMember: string) {
  const file: string = await readFile(cargoPath, "utf-8");
  const membersLines = REGEX.exec(file)![1]!.split("\n");
  membersLines.shift();
  membersLines.pop();
  const members = JSON.parse(
    `[${membersLines.map((x) => x.slice(0, -1)).join(",")}]`,
  );
  if (members.includes(newMember)) {
    console.error("already present");
    process.exit(0);
  }
  const newMembers = [...members, newMember];
  newMembers.sort((a, b) => (a > b ? 1 : -1));
  const rendered = `members = [\n${newMembers.map((x) => "    " + JSON.stringify(x) + ",\n").join("")}]`;
  const newFile = file.replace(REGEX, rendered);

  await writeFile(cargoPath, newFile);
}
