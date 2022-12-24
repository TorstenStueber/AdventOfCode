const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

function task1() {
  let s = input;
  for (let i = 0; i < 40; i++) {
    let n = "";
    for (let j = 0; j < s.length; j++) {
      if (j + 1 === s.length || s[j + 1] !== s[j]) {
        n += `1${s[j]}`;
      } else if (j + 2 === s.length || s[j + 2] !== s[j]) {
        n += `2${s[j]}`;
        j++;
      } else {
        n += `3${s[j]}`;
        j += 2;
      }
    }
    s = n;
  }

  return s.length;
}

function task2() {
  let s = input;
  for (let i = 0; i < 50; i++) {
    let n = "";
    for (let j = 0; j < s.length; j++) {
      if (j + 1 === s.length || s[j + 1] !== s[j]) {
        n += `1${s[j]}`;
      } else if (j + 2 === s.length || s[j + 2] !== s[j]) {
        n += `2${s[j]}`;
        j++;
      } else {
        n += `3${s[j]}`;
        j += 2;
      }
    }
    s = n;
  }

  return s.length;
}

console.log(task1());
console.log(task2());
