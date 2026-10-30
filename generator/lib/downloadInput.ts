import process from "node:process";
import util from "node:util";

export async function downloadInput({
  AOC,
  year,
  day,
}: {
  AOC: string;
  year: number;
  day: number;
}) {
  console.error({
    AOC: AOC?.slice(0, 5) + "..." + AOC.slice(-5),
    year,
    day,
  });

  const inputUrl = `https://adventofcode.com/${year}/day/${day}/input`;

  const response = await fetch(inputUrl, {
    headers: {
      Cookie: `session=${AOC}`,
    },
    signal: AbortSignal.timeout(10_000),
  });

  if (!response.ok) {
    throw new Error(
      `Failed to request input.\nurl: ${inputUrl}\nError: ${response.status}: ${response.statusText}`,
    );
  }

  const inputBody = await response.text();

  return inputBody;
}
