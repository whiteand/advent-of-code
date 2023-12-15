const str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

console.log(str);

let current = 0;
for (const part of str.split(",")) {
  for (let i = 0; i < part.length; i++) {
    const c = part.charCodeAt(i);
    current += c;
    current *= 17;
    current %= 256;
  }
  console.log(current);
  current = 0;
}
