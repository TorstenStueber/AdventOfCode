const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  let x = 0;
  let y = 0;
  const dirs = ["N", "W", "S", "E"];
  let offset = { N: [0, -1], W: [-1, 0], S: [0, 1], E: [1, 0] };
  let dir = 3;

  input.forEach((line) => {
    switch (line.slice(0, 1)) {
      case "F":
        x += offset[dirs[dir]][0] * Number(line.slice(1));
        y += offset[dirs[dir]][1] * Number(line.slice(1));
        break;
      case "N":
      case "S":
      case "E":
      case "W":
        x += offset[line.slice(0, 1)][0] * Number(line.slice(1));
        y += offset[line.slice(0, 1)][1] * Number(line.slice(1));
        break;
      case "L":
        dir += Number(line.slice(1)) / 90;
        if (dir >= 4) dir -= 4;
        break;
      case "R":
        dir -= Number(line.slice(1)) / 90;
        if (dir < 0) dir += 4;
        break;
    }
  });

  return Math.abs(x) + Math.abs(y);
}

function task2() {
  let x = 0;
  let y = 0;
  let wx = 10;
  let wy = -1;
  let offset = { N: [0, -1], W: [-1, 0], S: [0, 1], E: [1, 0] };

  input.forEach((line) => {
    switch (line.slice(0, 1)) {
      case "F":
        x += wx * Number(line.slice(1));
        y += wy * Number(line.slice(1));
        break;
      case "N":
      case "S":
      case "E":
      case "W":
        wx += offset[line.slice(0, 1)][0] * Number(line.slice(1));
        wy += offset[line.slice(0, 1)][1] * Number(line.slice(1));
        break;
      case "L":
      case "R":
        let angle = line.slice(0, 1) === "R" ? Number(line.slice(1)) / 90 : -Number(line.slice(1)) / 90;
        if (angle < 0) angle += 4;
        switch (angle) {
          case 0:
            break;
          case 1:
            {
              const t = -wy;
              wy = wx;
              wx = t;
            }
            break;
          case 2:
            wx = -wx;
            wy = -wy;
            break;
          case 3:
            {
              const t = wy;
              wy = -wx;
              wx = t;
            }
            break;
        }
        break;
    }
  });

  return Math.abs(x) + Math.abs(y);
}

console.log(task1());
console.log(task2());
