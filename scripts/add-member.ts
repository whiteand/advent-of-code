import { readFile } from 'node:fs/promises'
import process from 'node:process'
const REGEX = /members = (\[\n(\s+".*?".\n)+?\])/g

const [year, day] = Deno.args
const file: string = await Deno.readTextFile('Cargo.toml')
const membersLines = REGEX.exec(file)[1].split('\n');
membersLines.shift()
membersLines.pop()
const members= JSON.parse(`[${membersLines.map(x => x.slice(0, -1)).join(',')}]`)
const newMember = `y${year}/d${day.padStart(2, '0')}`
if (members.includes(newMember)) {
    console.log('already present')
    process.exit(0);
}
const newMembers = [...members, newMember];
newMembers.sort((a,b) => a > b ? 1 : -1)
const rendered = `members = [\n${newMembers.map(x => '    ' + JSON.stringify(x) + ',\n').join('')}]`
const newFile = file.replace(REGEX, rendered)

await Deno.writeTextFile('Cargo.toml', newFile)