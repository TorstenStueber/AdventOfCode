const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => {
    const [
      _,
      n,
      speed,
      t1,
      t2,
    ] = /(\w+) can fly (\d+) km\/s for (\d+) seconds, but then must rest for (\d+) seconds\./.exec(a);
    return { n, speed: Number(speed), t1: Number(t1), t2: Number(t2) };
  });

function task1() {
  const t = 2503;
  return input.reduce((a, b) => {
    s = Math.floor(t / (b.t1 + b.t2)) * b.t1 * b.speed + Math.min(b.t1, t % (b.t1 + b.t2)) * b.speed;
    return Math.max(a, s);
  }, 0);
}

function task2() {
  const maxT = 2503;
  const points = {};
  for (let t = 1; t <= maxT; t++) {
    const pos = {};

    input.forEach((b) => {
      s = Math.floor(t / (b.t1 + b.t2)) * b.t1 * b.speed + Math.min(b.t1, t % (b.t1 + b.t2)) * b.speed;
      if (pos[s] === undefined) pos[s] = [];
      pos[s].push(b.n);
    });

    pos[Object.keys(pos).sort((a, b) => b - a)[0]].forEach((r) => {
      if (points[r] === undefined) points[r] = 0;
      points[r]++;
    });
  }

  return Object.values(points).sort((a, b) => b - a)[0];
}

console.log(task1());
console.log(task2());
