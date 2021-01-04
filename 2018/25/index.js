const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => line.split(",").map((a) => Number(a)));

function task1() {
  let left = [...input];
  let constellations = 0;
  while (left.length > 0) {
    const start = left.shift();
    const c = [start];
    constellations++;

    while (true) {
      const l = c.length;

      let remove = [];
      left.forEach(([x, y, z, t], i) => {
        const belongs = c.some(
          ([x2, y2, z2, t2]) => Math.abs(x - x2) + Math.abs(y - y2) + Math.abs(z - z2) + Math.abs(t - t2) <= 3
        );
        if (belongs) {
          c.push([x, y, z, t]);
          remove.push(i);
        }
      });

      left = left.filter((_, j) => !remove.includes(j));
      if (c.length === l) break;
    }
  }

  return constellations;
}

console.log(task1());
