const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function isAbba(line, i) {
  return i < line.length - 3 && line[i] === line[i + 3] && line[i + 1] === line[i + 2] && line[i] !== line[i + 1];
}

function task1() {
  let count = 0;

  input.forEach((line) => {
    let inside = false;
    let foundOutside = false;
    let failed = false;
    line.split("").forEach((a, i) => {
      if (a === "[") {
        inside = true;
      } else if (a === "]") {
        inside = false;
      } else if (isAbba(line, i)) {
        if (inside) {
          failed = true;
        } else {
          foundOutside = true;
        }
      }
    });

    if (!failed && foundOutside) count++;
  });

  return count;
}

function isAba(line, i) {
  return i < line.length - 2 && line[i] === line[i + 2] && line[i] !== line[i + 1];
}

function task2() {
  let count = 0;

  input.forEach((line) => {
    let inside = false;
    const found = {};
    line.split("").forEach((a, i) => {
      if (a === "[") {
        inside = true;
      } else if (a === "]") {
        inside = false;
      } else if (isAba(line, i)) {
        if (inside) {
          found[`${line[i + 1]}${line[i]}`] = { ...found[`${line[i + 1]}${line[i]}`], inside: true };
        } else {
          found[`${line[i]}${line[i + 1]}`] = { ...found[`${line[i]}${line[i + 1]}`], outside: true };
        }
      }
    });

    const correct = Object.values(found).some(({ inside, outside }) => inside && outside);
    if (correct) count++;
  });

  return count;
}

console.log(task1());
console.log(task2());
