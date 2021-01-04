const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function findHits(x, y) {
  const hits = [];
  for (let yy = 0; yy < input.length; yy++) {
    for (let xx = 0; xx < input[0].length; xx++) {
      if (input[yy][xx] === "#" && (yy !== y || xx !== x)) {
        const dx = xx - x;
        const dy = yy - y;
        if (!hits.some(([hx, hy]) => hx * dy === hy * dx && hx * dx >= 0 && hy * dy >= 0)) {
          hits.push([dx, dy]);
        }
      }
    }
  }
  return hits.length;
}

function findBest() {
  let max = 0;
  let bx, by;
  for (let yy = 0; yy < input.length; yy++) {
    for (let xx = 0; xx < input[0].length; xx++) {
      if (input[yy][xx] === "#") {
        const hits = findHits(xx, yy);
        if (max < hits) {
          max = hits;
          bx = xx;
          by = yy;
        }
      }
    }
  }

  return { max, bx, by };
}

function angle(dx, dy) {
  if (dy === 0) {
    return dx > 0 ? Math.PI / 2 : (3 * Math.PI) / 2;
  }

  const tan = Math.atan(dx / -dy);
  const tan2 = dy > 0 ? tan + Math.PI : tan;
  return tan2 < 0 ? tan2 + Math.PI * 2 : tan2;
}

function task1() {
  const { max } = findBest();
  return max;
}

function findShot(ang, bx, by, field, initial) {
  const hits = [];
  for (let yy = 0; yy < field.length; yy++) {
    for (let xx = 0; xx < field[0].length; xx++) {
      if (field[yy][xx] === "#" && (yy !== by || xx !== bx)) {
        const dx = xx - bx;
        const dy = yy - by;
        const a = angle(dx, dy) - ang;
        const wrap = initial ? a < 0 : a <= 0;
        hits.push([xx, yy, wrap ? a + Math.PI * 2 : a, dx * dx + dy * dy]);
      }
    }
  }
  const [x, y] = hits.sort((a, b) => (a[2] !== b[2] ? a[2] - b[2] : a[3] - b[3]))[0];
  return [x, y];
}

function task2() {
  const { bx, by } = findBest();
  let ang = 0;
  let field = [...input];

  for (let i = 0; i < 200; i++) {
    const [x, y] = findShot(ang, bx, by, field, i === 0);
    if (i === 199) return x * 100 + y;
    field[x][y] = "S";
    ang = angle(x - bx, y - by);
  }
}

console.log(task1());
console.log(task2());
