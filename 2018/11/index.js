const { readFileSync } = require("fs");
const { join } = require("path");

const input = Number(readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n"));

function task1() {
  let max = -Infinity;
  let maxX;
  let maxY;
  for (let x = 1; x <= 298; x++) {
    for (let y = 1; y <= 298; y++) {
      let s = 0;
      for (let xx = 0; xx <= 2; xx++) {
        for (let yy = 0; yy <= 2; yy++) {
          const rackId = x + xx + 10;
          const powerLevel = (rackId * (y + yy) + input) * rackId;
          s += (Math.floor(powerLevel / 100) % 10) - 5;
        }
      }
      if (s > max) {
        max = s;
        maxX = x;
        maxY = y;
      }
    }
  }

  return `${maxX},${maxY}`;
}
function task2() {
  let max = -Infinity;
  let maxX;
  let maxY;
  let maxSize;
  let powerLevels = {};

  function pL(x, y) {
    const key = `${x};${y}`;
    if (powerLevels[key] === undefined) {
      const rackId = x + 10;
      const powerLevel = (rackId * y + input) * rackId;
      powerLevels[key] = (Math.floor(powerLevel / 100) % 10) - 5;
    }
    return powerLevels[key];
  }

  for (let x = 1; x <= 300; x++) {
    console.log(x);
    for (let y = 1; y <= 300; y++) {
      let s = 0;
      for (let size = 1; size <= 301 - x && size <= 301 - y; size++) {
        for (let d = 0; d < size - 1; d++) {
          s += pL(x + d, y + size - 1);
          s += pL(x + size - 1, y + d);
        }
        s += pL(x + size - 1, y + size - 1);

        if (s > max) {
          max = s;
          maxX = x;
          maxY = y;
          maxSize = size;
        }
      }
    }
  }

  return `${maxX},${maxY},${maxSize}`;
}

console.log(task1());
console.log(task2());
