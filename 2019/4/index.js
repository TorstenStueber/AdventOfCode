const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("-")
  .map((a) => parseInt(a, 10));

function task1() {
  let count = 0;
  for (let i = input[0]; i <= input[1]; i++) {
    const s = String(i).split("");
    let hasDouble = false;
    let isIncreasing = true;
    s.forEach((l, j) => {
      if (j < s.length - 1 && s[j + 1] === l) hasDouble = true;
      if (j < s.length - 1 && Number(l) > Number(s[j + 1])) isIncreasing = false;
    });
    if (hasDouble && isIncreasing) count++;
  }
  return count;
}

function task2() {
  let count = 0;
  for (let i = input[0]; i <= input[1]; i++) {
    const s = String(i).split("");
    let hasDouble = false;
    let isIncreasing = true;
    s.forEach((l, j) => {
      if (j < s.length - 1 && s[j + 1] === l) {
        const notBefore = j <= 0 || s[j - 1] !== l;
        const notAfter = j >= s.length - 2 || s[j + 2] !== l;
        if (notBefore && notAfter) hasDouble = true;
      }
      if (j < s.length - 1 && Number(l) > Number(s[j + 1])) isIncreasing = false;
    });
    if (hasDouble && isIncreasing) count++;
  }
  return count;
}

console.log(task1());
console.log(task2());
