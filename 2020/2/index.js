const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => /^(\d+)-(\d+) (.): (.+)$/.exec(a));

function task1() {
  let count = 0;
  input.forEach((line) => {
    const n = line[4].split("").filter((letter) => letter === line[3]).length;
    if (n >= Number(line[1]) && n <= Number(line[2])) count++;
  });
  return count;
}

function task2() {
  let count = 0;
  input.forEach((line) => {
    const match1 = line[4][Number(line[1]) - 1] === line[3];
    const match2 = line[4][Number(line[2]) - 1] === line[3];
    if ((match1 && !match2) || (!match1 && match2)) count++;
  });
  return count;
}

console.log(task1());
console.log(task2());
