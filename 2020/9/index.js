const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  for (let i = 25; i < input.length; i++) {
    let found = false;
    for (let j = 0; j < 25; j++) {
      for (let k = 0; k < 25; k++) {
        if (j !== k && Number(input[i - j - 1]) + Number(input[i - k - 1]) === Number(input[i])) found = true;
      }
    }
    if (!found) return Number(input[i]);
  }
}

function task2() {
  const invalid = task1();

  for (let i = 0; i < input.length; i++) {
    for (let j = 2; j + i <= input.length; j++) {
      let sum = 0;
      let min = Infinity;
      let max = 0;
      for (let k = 0; k < j; k++) {
        sum += Number(input[i + k]);
        min = Math.min(min, Number(input[i + k]));
        max = Math.max(max, Number(input[i + k]));
      }
      if (sum === invalid) return min + max;
    }
  }
}

console.log(task1());
console.log(task2());
