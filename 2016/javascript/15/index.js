const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => {
    const [_, n, s] = /Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+)\./.exec(line);
    return { n: Number(n), s: Number(s) };
  });

function find(input) {
  for (let i = 0; true; i++) {
    const success = input.every(({ n, s }, j) => {
      return (j + i + 1 + s) % n === 0;
    });
    if (success) return i;
  }
}

function task1() {
  return find(input);
}

function task2() {
  return find([...input, { n: 11, s: 0 }]);
}

console.log(task1());
console.log(task2());
