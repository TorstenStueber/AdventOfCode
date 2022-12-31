const { readFileSync } = require("fs");
const { join } = require("path");
const { createHash } = require("crypto");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

function task1() {
  const result = [];
  let hash;
  for (let i = 0; result.length < 8; i++) {
    if (i % 1000000 === 0) console.log(">", i, result.length);
    hash = createHash("MD5");
    hash.update(`${input}${i}`);
    const digest = hash.digest("hex");
    if (digest.startsWith("00000")) result.push(digest[5]);
  }
  return result.join("");
}

function task2() {
  const result = [];
  let hash;
  let found = 0;
  for (let i = 0; found < 8; i++) {
    if (i % 1000000 === 0) console.log(">", i, found);
    hash = createHash("MD5");
    hash.update(`${input}${i}`);
    const digest = hash.digest("hex");
    if (digest.startsWith("00000")) {
      const pos = parseInt(digest[5], 16);
      if (pos < 8 && result[pos] === undefined) {
        result[pos] = digest[6];
        found++;
      }
    }
  }
  return result.join("");
}

console.log(task1());
console.log(task2());
