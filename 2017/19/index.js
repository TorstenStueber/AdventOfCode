const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").split("\n");

const dirs = [
  [0, 1],
  [-1, 0],
  [0, -1],
  [1, 0],
];

function walk() {
  let x = input[0].indexOf("|");
  let y = 0;
  let dir = 0;
  let string = "";
  let steps = 0;
  while (true) {
    if (y < 0 || y > input.length || x < 0 || x > input[y].length || input[y][x] === " ") {
      return { string, steps };
    }
    if (input[y][x] === "+") {
      const nextDir1 = (dir + 1) % 4;
      const nextDir2 = (dir + 3) % 4;
      const [xx1, yy1] = dirs[nextDir1];
      const xxx1 = x + xx1;
      const yyy1 = y + yy1;
      if (yyy1 < 0 || yyy1 > input.length || xxx1 < 0 || xxx1 > input[yyy1].length || input[yyy1][xxx1] === " ") {
        dir = nextDir2;
      } else {
        dir = nextDir1;
      }
    } else if (/\w/.exec(input[y][x])) {
      string += input[y][x];
    }

    const [xx, yy] = dirs[dir];
    x += xx;
    y += yy;
    steps++;
  }
}

function task1() {
  return walk().string;
}

function task2() {
  return walk().steps;
}

console.log(task1());
console.log(task2());
