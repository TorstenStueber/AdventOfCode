const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("");

function type(line, i) {
  return i < 0 || i >= line.length || line[i] === "." ? "." : "^";
}

function nextLine(line) {
  return line.map((a, i) => {
    const trapL = type(line, i - 1) === type(line, i) && type(line, i) !== type(line, i + 1);
    const trapR = type(line, i - 1) !== type(line, i) && type(line, i) === type(line, i + 1);
    return trapL || trapR ? "^" : ".";
  });
}

function task1() {
  const lines = [input];
  while (lines.length !== 40) {
    lines.push(nextLine(lines[lines.length - 1]));
  }

  let c = 0;
  lines.forEach((line) =>
    line.forEach((a) => {
      if (a === ".") c++;
    })
  );

  return c;
}

function task2() {
  const lines = [input];
  while (lines.length !== 400000) {
    lines.push(nextLine(lines[lines.length - 1]));
  }

  let c = 0;
  lines.forEach((line) =>
    line.forEach((a) => {
      if (a === ".") c++;
    })
  );

  return c;
}

console.log(task1());
console.log(task2());
