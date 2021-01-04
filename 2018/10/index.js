const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) =>
    /position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>/
      .exec(a)
      .slice(1)
      .map((b) => Number(b))
  );

function step(state) {
  return state.map(([xp, yp, xv, yv]) => [xp + xv, yp + yv, xv, yv]);
}

function run() {
  let minSize = Infinity;
  let minState;
  let minIndex;

  let state = input;

  for (let i = 0; true; i++) {
    let minX = Infinity;
    let maxX = -Infinity;
    let minY = Infinity;
    let maxY = -Infinity;

    state.forEach(([x, y]) => {
      minX = Math.min(minX, x);
      minY = Math.min(minY, y);
      maxX = Math.max(maxX, x);
      maxY = Math.max(maxY, y);
    });

    const size = maxX - minX + (maxY - minY);
    if (size < minSize) {
      minSize = size;
      minState = state;
      minIndex = i;
    } else break;

    state = step(state);
  }

  let minX = Infinity;
  let maxX = -Infinity;
  let minY = Infinity;
  let maxY = -Infinity;

  minState.forEach(([x, y]) => {
    minX = Math.min(minX, x);
    minY = Math.min(minY, y);
    maxX = Math.max(maxX, x);
    maxY = Math.max(maxY, y);
  });

  const field = [];
  for (y = minY; y <= maxY; y++) {
    field.push([]);
    for (x = minX; x <= maxX; x++) {
      field[y - minY][x - minX] = ".";
    }
  }
  minState.forEach(([x, y]) => {
    field[y - minY][x - minX] = "#";
  });

  return { message: field.map((a) => a.join("")).join("\n"), index: minIndex };
}

function task1() {
  return run().message;
}
function task2() {
  return run().index;
}

console.log(task1());
console.log(task2());
