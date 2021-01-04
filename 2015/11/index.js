const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

function next(p) {
  let carry = true;
  let n = [];
  for (let i = p.length - 1; i >= 0; i--) {
    if (carry) {
      n.push(p[i] === "z" ? "a" : String.fromCharCode(p.charCodeAt(i) + 1));
      carry = p[i] === "z";
    } else {
      n.push(p[i]);
    }
  }
  return n.reverse().join("");
}

function findNext(p) {
  function match() {
    let found = false;
    for (let i = 0; i < 24; i++) {
      const j = i + "a".charCodeAt(0);
      if (p.indexOf(`${String.fromCharCode(j)}${String.fromCharCode(j + 1)}${String.fromCharCode(j + 2)}`) >= 0)
        found = true;
    }
    if (!found) return false;

    if (p.indexOf("o") >= 0 || p.indexOf("i") >= 0 || p.indexOf("l") >= 0) return false;

    found = false;
    for (let i = 0; i < p.length - 3; i++) {
      for (let j = i + 2; j < p.length - 1; j++) {
        if (p[i] === p[i + 1] && p[j] === p[j + 1] && p[i] !== p[j]) found = true;
      }
    }
    if (!found) return false;

    return true;
  }

  while (!match()) {
    p = next(p);
  }

  return p;
}

function task1() {
  return findNext(input);
}

function task2() {
  return findNext(next(findNext(input)));
}

console.log(task1());
console.log(task2());
