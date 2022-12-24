const { readFileSync } = require("fs");
const { join } = require("path");
const { createHash } = require("crypto");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  const area = [];
  for (let i = 0; i < 1000; i++) area.push([]);

  input.forEach((line) => {
    [_, instr, x1, y1, x2, y2] = /(.+) (\d+),(\d+) through (\d+),(\d+)/.exec(line);
    for (x = Number(x1); x <= Number(x2); x++)
      for (y = Number(y1); y <= Number(y2); y++) {
        if (instr === "turn off") {
          area[y][x] = ".";
        } else if (instr === "turn on") {
          area[y][x] = "#";
        } else if (instr === "toggle") {
          area[y][x] = area[y][x] === "#" ? "." : "#";
        }
      }
  });

  return area.reduce((a, b) => a + b.filter((x) => x === "#").length, 0);
}

function task2() {
  const area = [];
  for (let i = 0; i < 1000; i++) area.push([]);

  input.forEach((line) => {
    [_, instr, x1, y1, x2, y2] = /(.+) (\d+),(\d+) through (\d+),(\d+)/.exec(line);
    for (x = Number(x1); x <= Number(x2); x++)
      for (y = Number(y1); y <= Number(y2); y++) {
        if (instr === "turn off") {
          area[y][x] = area[y][x] !== undefined ? Math.max(0, area[y][x] - 1) : 0;
        } else if (instr === "turn on") {
          area[y][x] = area[y][x] !== undefined ? area[y][x] + 1 : 1;
        } else if (instr === "toggle") {
          area[y][x] = area[y][x] !== undefined ? area[y][x] + 2 : 2;
        }
      }
  });

  return area.reduce((a, b) => a + b.reduce((aa, bb) => aa + bb, 0), 0);
}

console.log(task1());
console.log(task2());
