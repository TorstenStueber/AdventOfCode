const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  let mask = "";
  let values = {};
  input.forEach((line) => {
    if (line.startsWith("mask")) {
      mask = line.slice(7).padStart(36, "0").split("");
    } else {
      const x = /mem\[(\d+)\] = (\d+)/.exec(line);
      const value = Number(x[2]).toString(2).padStart(36, "0").split("");
      values[x[1]] = parseInt(value.map((p, l) => (mask[l] === "X" ? p : mask[l])).join(""), 2);
    }
  });
  return Object.values(values).reduce((a, b) => a + b, 0);
}

function task2() {
  let mask = "";
  let values = {};

  function writeMask(start, i, address, mask, value) {
    if (i === mask.length) {
      values[parseInt(start, 2)] = value;
    } else {
      if (mask[i] === "X") {
        writeMask(start + "0", i + 1, address, mask, value);
        writeMask(start + "1", i + 1, address, mask, value);
      } else if (mask[i] === "0") {
        writeMask(start + address[i], i + 1, address, mask, value);
      } else {
        writeMask(start + "1", i + 1, address, mask, value);
      }
    }
  }

  input.forEach((line) => {
    if (line.startsWith("mask")) {
      mask = line.slice(7).padStart(36, "0").split("");
    } else {
      const x = /mem\[(\d+)\] = (\d+)/.exec(line);
      const value = Number(x[2]);
      const address = Number(x[1]).toString(2).padStart(36, "0").split("");
      writeMask("", 0, address, mask, value);
    }
  });
  return Object.values(values).reduce((a, b) => a + b, 0);
}

console.log(task1());
console.log(task2());
