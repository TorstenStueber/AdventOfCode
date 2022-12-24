const { readFileSync } = require("fs");
const { join } = require("path");
const { createHash } = require("crypto");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  return input.filter((line) => {
    if (line.split("").filter((a) => "aeiou".indexOf(a) >= 0).length < 3) return false;
    if (!line.split("").some((a, i) => i < line.length - 1 && line[i + 1] === a)) return false;
    if (["ab", "cd", "pq", "xy"].some((c) => line.indexOf(c) >= 0)) return false;
    return true;
  }).length;
}

function task2() {
  return input.filter((line) => {
    if (!line.split("").some((_, i) => i < line.length - 3 && line.slice(i + 2).indexOf(line.slice(i, i + 2)) >= 0))
      return false;
    if (!line.split("").some((a, i) => i < line.length - 2 && line[i + 2] === a)) return false;
    return true;
  }).length;
}

console.log(task1());
console.log(task2());
