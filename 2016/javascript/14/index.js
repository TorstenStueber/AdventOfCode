const { readFileSync } = require("fs");
const { join } = require("path");
const { createHash } = require("crypto");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

const hashes = [];

function hash(i) {
  if (hashes[i] === undefined) {
    md5 = createHash("MD5");
    md5.update(`${input}${i}`);
    hashes[i] = md5.digest("hex");
  }
  return hashes[i];
}

function hasThree(s) {
  for (let i = 0; i < s.length - 2; i++) {
    if (s[i] === s[i + 1] && s[i] === s[i + 2]) return s[i];
  }
}

function hasFive(s, a) {
  for (let i = 0; i < s.length - 4; i++) {
    let fits = true;
    for (let k = 0; k < 5; k++) {
      if (s[i + k] !== a) {
        fits = false;
        break;
      }
    }
    if (fits) return true;
  }
  return false;
}

function task1() {
  let foundCount = 0;
  for (let i = 0; true; i++) {
    const digest = hash(i);
    const a = hasThree(digest);
    if (a !== undefined) {
      for (let j = 1; j <= 1000; j++) {
        if (hasFive(hash(i + j), a)) {
          foundCount++;
          if (foundCount === 64) return i;
          break;
        }
      }
    }
  }
}

const hashes2 = [];

function hash2(i) {
  if (hashes2[i] === undefined) {
    let current = `${input}${i}`;
    for (let i = 0; i < 2017; i++) {
      md5 = createHash("MD5");
      md5.update(current);
      current = md5.digest("hex");
    }
    hashes2[i] = current;
  }
  return hashes2[i];
}

function task2() {
  let foundCount = 0;
  for (let i = 0; true; i++) {
    if (i % 1000 === 0) console.log(">", i, foundCount);
    const digest = hash2(i);
    const a = hasThree(digest);
    if (a !== undefined) {
      for (let j = 1; j <= 1000; j++) {
        if (hasFive(hash2(i + j), a)) {
          foundCount++;
          if (foundCount === 64) return i;
          break;
        }
      }
    }
  }
}

console.log(task1());
console.log(task2());
