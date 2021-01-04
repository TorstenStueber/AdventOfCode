const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  return input.reduce((a, b) => {
    let count = 0;
    while (b.indexOf('\\"') >= 0) {
      count++;
      b = b.replace('\\"', "");
    }
    while (b.indexOf("\\\\") >= 0) {
      count++;
      b = b.replace("\\\\", "");
    }
    while (/\\x[0-9a-f][0-9a-f]/.exec(b)) {
      count += 3;
      b = b.replace(/\\x[0-9a-f][0-9a-f]/, "");
    }
    return a + count + 2;
  }, 0);
}

function task2() {
  return input.reduce((a, b) => {
    return a + b.split("").filter((x) => x === '"' || x === "\\").length + 2;
  }, 0);
}

console.log(task1());
console.log(task2());
