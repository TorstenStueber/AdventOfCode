const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

function task1() {
  return input.split("").reduce((a, b) => a + (b === "(" ? 1 : -1), 0);
}

function task2() {
  let floor = 0;
  return (
    1 +
    input.split("").findIndex((b) => {
      floor += b === "(" ? 1 : -1;
      return floor < 0;
    })
  );
}

console.log(task1());
console.log(task2());
