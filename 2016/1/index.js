const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split(", ");

const dirs = [
  [0, -1],
  [1, 0],
  [0, 1],
  [-1, 0],
];
function task1() {
  let dir = 0;
  let x = 0;
  let y = 0;
  input.forEach((c) => {
    if (c[0] === "L") {
      dir--;
      if (dir < 0) dir += 4;
    } else {
      dir++;
      if (dir > 3) dir -= 4;
    }
    x += dirs[dir][0] * Number(c.slice(1));
    y += dirs[dir][1] * Number(c.slice(1));
  });

  return Math.abs(x) + Math.abs(y);
}

function task2() {
  let dir = 0;
  let x = 0;
  let y = 0;
  const visited = new Set();

  let result;
  input.some((c) => {
    if (c[0] === "L") {
      dir--;
      if (dir < 0) dir += 4;
    } else {
      dir++;
      if (dir > 3) dir -= 4;
    }

    let d = Number(c.slice(1));
    while (d > 0) {
      const key = `${x};${y}`;
      if (visited.has(key)) {
        result = Math.abs(x) + Math.abs(y);
        return true;
      }
      visited.add(key);
      x += dirs[dir][0];
      y += dirs[dir][1];
      d--;
    }
  });

  return result;
}

console.log(task1());
console.log(task2());
