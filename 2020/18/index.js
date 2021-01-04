const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function ev(line, start) {
  let value;
  let op;

  while (true) {
    while (start < line.length && line[start] === " ") start++;

    let d;
    if (line[start] === "(") {
      const r = ev(line, start + 1);
      d = r.v;
      start = r.s;
    } else {
      d = Number(/(\d+)/.exec(line.slice(start))[1]);
    }

    if (op === undefined) {
      value = d;
    } else if (op === "+") {
      value += d;
    } else {
      value *= d;
    }

    while (start < line.length && line[start] !== ")" && line[start] !== "+" && line[start] !== "*") start++;
    if (start >= line.length) return { v: value, s: start };

    if (line[start] === ")") {
      return { v: value, s: start + 1 };
    } else if (line[start] === "+") {
      op = "+";
    } else {
      op = "*";
    }
    start += 2;
  }
}

function task1() {
  return input.reduce((a, b) => a + ev(b, 0).v, 0);
}

function ev2(line, start) {
  let value;
  let term;
  let inTerm = false;

  while (true) {
    while (start < line.length && line[start] === " ") start++;

    let d;
    if (line[start] === "(") {
      const r = ev2(line, start + 1);
      d = r.v;
      start = r.s;
    } else {
      d = Number(/(\d+)/.exec(line.slice(start))[1]);
    }

    while (start < line.length && line[start] !== ")" && line[start] !== "+" && line[start] !== "*") start++;

    if (inTerm || (start < line.length && line[start] === "+")) {
      if (!inTerm) {
        inTerm = true;
        term = d;
      } else {
        term += d;
        if (start >= line.length || line[start] !== "+") {
          inTerm = false;
          if (value === undefined) {
            value = term;
          } else {
            value *= term;
          }
        }
      }
    } else {
      if (value === undefined) {
        value = d;
      } else {
        value *= d;
      }
    }

    if (start >= line.length) return { v: value, s: start };

    if (line[start] === ")") {
      return { v: value, s: start + 1 };
    }
    start += 2;
  }
}

function task2() {
  return input.reduce((a, b) => a + ev2(b, 0).v, 0);
}

console.log(task1());
console.log(task2());
