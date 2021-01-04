const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line, i) => {
    const [_, x, y, z, r] = /pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)/.exec(line);
    return { i, x: Number(x), y: Number(y), z: Number(z), r: Number(r) };
  });

function task1() {
  const { i } = [...input].sort(({ r: r1 }, { r: r2 }) => r2 - r1)[0];
  return input.reduce(
    (a, { x, y, z }) =>
      a + (Math.abs(input[i].x - x) + Math.abs(input[i].y - y) + Math.abs(input[i].z - z) <= input[i].r ? 1 : 0),
    0
  );
}

function intersectsCube({ x, y, z, r }, { minX, maxX, minY, maxY, minZ, maxZ }) {
  if (x < minX) {
    r -= minX - x;
  } else if (x > maxX) {
    r -= x - maxX;
  }

  if (y < minY) {
    r -= minY - y;
  } else if (y > maxY) {
    r -= y - maxY;
  }

  if (z < minZ) {
    r -= minZ - z;
  } else if (z > maxZ) {
    r -= z - maxZ;
  }
  return r >= 0;
}

function task2() {
  let minX = Infinity;
  let maxX = -Infinity;
  let minY = Infinity;
  let maxY = -Infinity;
  let minZ = Infinity;
  let maxZ = -Infinity;

  let best = 0;
  let bestX;
  let bestY;
  let bestZ;

  input.forEach(({ x, y, z, r }) => {
    minX = Math.min(minX, x - r);
    maxX = Math.max(maxX, x + r);
    minY = Math.min(minY, y - r);
    maxY = Math.max(maxY, y + r);
    minZ = Math.min(minZ, z - r);
    maxZ = Math.max(maxZ, z + r);
  });

  function add(minX, maxX, minY, maxY, minZ, maxZ) {
    if (minX > maxX || minY > maxY || minZ > maxZ) return;
    const d = input.filter((bot) => intersectsCube(bot, { minX, maxX, minY, maxY, minZ, maxZ })).length;
    if (todo[d] === undefined) todo[d] = [];
    todo[d].push({ minX, maxX, minY, maxY, minZ, maxZ });
  }

  todo = { [input.length]: [{ minX, maxX, minY, maxY, minZ, maxZ }] };
  while (true) {
    const d = Number(Object.keys(todo).sort((a, b) => Number(b) - Number(a))[0]);
    if (d < best) break;

    const { minX, maxX, minY, maxY, minZ, maxZ } = todo[d].shift();
    if (todo[d].length === 0) delete todo[d];

    if (minX === maxX && minY === maxY && minZ === maxZ) {
      const d = input.filter(({ x, y, z, r }) => Math.abs(x - minX) + Math.abs(y - minY) + Math.abs(z - minZ) <= r)
        .length;
      if (d > best) {
        best = d;
        bestX = minX;
        bestY = minY;
        bestZ = minZ;
      } else if (d === best) {
        if (Math.abs(minX) + Math.abs(minY) + Math.abs(minZ) < Math.abs(bestX) + Math.abs(bestY) + Math.abs(bestZ)) {
          bestX = minX;
          bestY = minY;
          bestZ = minZ;
        }
      }
    } else {
      const midX = Math.floor((minX + maxX) / 2);
      const midY = Math.floor((minY + maxY) / 2);
      const midZ = Math.floor((minZ + maxZ) / 2);
      add(minX, midX, minY, midY, minZ, midZ);
      add(minX, midX, minY, midY, midZ + 1, maxZ);
      add(minX, midX, midY + 1, maxY, minZ, midZ);
      add(minX, midX, midY + 1, maxY, midZ + 1, maxZ);
      add(midX + 1, maxX, minY, midY, minZ, midZ);
      add(midX + 1, maxX, minY, midY, midZ + 1, maxZ);
      add(midX + 1, maxX, midY + 1, maxY, minZ, midZ);
      add(midX + 1, maxX, midY + 1, maxY, midZ + 1, maxZ);
    }
  }

  return Math.abs(bestX) + Math.abs(bestY) + Math.abs(bestZ);
}

console.log(task1());
console.log(task2());
