const { readFileSync, chmod } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

const [_, row, column] = /[^\d]+(\d+)[^\d]+(\d+)/.exec(input);

console.log(row, column);

function task1() {
  let current = 20151125;
  let r = 1;
  let c = 1;
  while (r !== Number(row) || c !== Number(column)) {
    if (r === 1) {
      r = c + 1;
      c = 1;
    } else {
      r--;
      c++;
    }
    current = (current * 252533) % 33554393;
  }

  return current;
}

console.log(task1());
