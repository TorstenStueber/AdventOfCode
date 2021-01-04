const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  const d = { 1: 0, 3: 0, 2: 0 };
  const s = [0, ...input.map((x) => Number(x)).sort((a, b) => a - b)];
  s.push(s[s.length - 1] + 3);
  s.forEach((c, i) => {
    if (i > 0) {
      d[c - s[i - 1]]++;
    }
  });
  return d[3] * d[1];
}

function task2() {
  const s = [0, ...input.map((x) => Number(x)).sort((a, b) => a - b)];
  s.push(s[s.length - 1] + 3);
  c = [];
  c[s.length - 1] = 1;
  for (let i = s.length - 2; i >= 0; i--) {
    let cc = c[i + 1];
    if (i + 2 < s.length && s[i + 2] - s[i] <= 3) cc += c[i + 2];
    if (i + 3 < s.length && s[i + 3] - s[i] <= 3) cc += c[i + 3];
    c[i] = cc;
  }

  return c[0];
}

console.log(task1());
console.log(task2());
