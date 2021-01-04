const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => {
    const [_, id, x, y, w, h] = /#(\d+) @ (\d+),(\d+): (\d+)x(\d+)/.exec(line);
    return { id, x: Number(x), y: Number(y), w: Number(w), h: Number(h) };
  });

function makeField() {
  const field = {};
  input.forEach(({ x, y, w, h }) => {
    for (let xx = 0; xx < w; xx++) {
      for (let yy = 0; yy < h; yy++) {
        const key = `${x + xx};${y + yy}`;
        if (field[key] === undefined) field[key] = 0;
        field[key]++;
      }
    }
  });

  return field;
}

function task1() {
  const field = makeField();

  return Object.values(field).filter((v) => v >= 2).length;
}

function task2() {
  const field = makeField();
  const found = input.find(({ x, y, w, h }) => {
    for (let xx = 0; xx < w; xx++) {
      for (let yy = 0; yy < h; yy++) {
        const key = `${x + xx};${y + yy}`;
        if (field[key] > 1) return false;
      }
    }
    return true;
  });

  return found.id;
}

console.log(task1());
console.log(task2());
