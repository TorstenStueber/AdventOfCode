const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .split("\n")
  .map((a) => a.split("/").map((b) => Number(b)));

function task1() {
  let max = 0;
  function explore(current, left, strength) {
    max = Math.max(max, strength);
    input.forEach(([a, b], i) => {
      if (!left.includes(i)) return;
      if (a === current) {
        explore(
          b,
          left.filter((j) => j !== i),
          strength + a + b
        );
      }
      if (b === current) {
        explore(
          a,
          left.filter((j) => j !== i),
          strength + a + b
        );
      }
    });
  }

  explore(
    0,
    input.map((_, i) => i),
    0
  );

  return max;
}

function task2() {
  let max = 0;
  let maxLength = 0;
  function explore(current, left, strength, length) {
    if (length > maxLength) {
      max = strength;
      maxLength = length;
    } else if (length === maxLength) {
      max = Math.max(max, strength);
    }

    input.forEach(([a, b], i) => {
      if (!left.includes(i)) return;
      if (a === current) {
        explore(
          b,
          left.filter((j) => j !== i),
          strength + a + b,
          length + 1
        );
      }
      if (b === current) {
        explore(
          a,
          left.filter((j) => j !== i),
          strength + a + b,
          length + 1
        );
      }
    });
  }

  explore(
    0,
    input.map((_, i) => i),
    0,
    0
  );

  return max;
}

console.log(task1());
console.log(task2());
