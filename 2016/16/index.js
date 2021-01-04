const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("");

function cs(length) {
  let curve = input;
  while (curve.length < length) {
    curve = [...curve, "0", ...[...curve].reverse().map((a) => (a === "0" ? "1" : "0"))];
  }

  curve = curve.slice(0, length);
  while (curve.length % 2 === 0) {
    const nextCurve = [];
    for (let i = 0; i < curve.length; i += 2) {
      nextCurve.push(curve[i] === curve[i + 1] ? "1" : "0");
    }
    curve = nextCurve;
  }

  return curve.join("");
}

function task1() {
  return cs(272);
}

function task2() {
  return cs(35651584);
}

console.log(task1());
console.log(task2());
