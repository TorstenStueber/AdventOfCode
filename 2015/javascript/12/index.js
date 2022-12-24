const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

function task1() {
  function add(d) {
    if (typeof d === "number") return d;
    if (typeof d === "string") return 0;
    if (d.length !== undefined) return d.reduce((a, b) => a + add(b), 0);
    return Object.values(d).reduce((a, b) => a + add(b), 0);
  }

  return add(JSON.parse(input));
}

function task2() {
  function add(d) {
    if (typeof d === "number") return d;
    if (typeof d === "string") return 0;
    if (d.length !== undefined) return d.reduce((a, b) => a + add(b), 0);
    if (Object.values(d).indexOf("red") >= 0) return 0;
    return Object.values(d).reduce((a, b) => a + add(b), 0);
  }

  return add(JSON.parse(input));
}

console.log(task1());
console.log(task2());
