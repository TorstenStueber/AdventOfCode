const { readFileSync } = require("fs");
const { join } = require("path");
const { createHash } = require("crypto");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

function task1() {
  for (let i = 0; ; i++) {
    const hash = createHash("MD5");
    hash.update(`${input}${i}`);
    if (hash.digest("hex").startsWith("00000")) return i;
  }
}

function task2() {
  for (let i = 0; ; i++) {
    if (!(i % 1000000)) console.log(">", i);
    const hash = createHash("MD5");
    hash.update(`${input}${i}`);
    if (hash.digest("hex").startsWith("000000")) return i;
  }
}

console.log(task1());
console.log(task2());
