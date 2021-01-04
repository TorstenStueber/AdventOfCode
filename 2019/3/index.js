const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n");

function trace(line) {
  const s = new Set();
  const lowestStep = {};
  let x = 0;
  let y = 0;
  let steps = 0;
  s.add("0,0");
  line.split(",").forEach((c) => {
    let map;
    switch (c[0]) {
      case "U":
        map = () => {
          y--;
        };
        break;
      case "L":
        map = () => {
          x--;
        };
        break;
      case "R":
        map = () => {
          x++;
        };
        break;
      case "D":
        map = () => {
          y++;
        };
        break;
    }

    for (let l = Number(c.slice(1)); l--; ) {
      map();
      steps++;
      const pos = `${x},${y}`;
      s.add(pos);
      if (!lowestStep[pos]) lowestStep[pos] = steps;
    }
  });

  return { s, lowestStep };
}

function task1() {
  const { s: s1 } = trace(input[0]);
  const { s: s2 } = trace(input[1]);
  let d = Infinity;
  Array.from(s1).forEach((p) => {
    if (s2.has(p) && p !== "0,0") {
      const [x, y] = p.split(",");
      const dd = Math.abs(Number(x)) + Math.abs(Number(y));
      if (dd < d) d = dd;
    }
  });
  return d;
}

function task2() {
  const { s: s1, lowestStep: l1 } = trace(input[0]);
  const { s: s2, lowestStep: l2 } = trace(input[1]);
  let d = Infinity;
  Array.from(s1).forEach((p) => {
    if (s2.has(p) && p !== "0,0") {
      const dd = l1[p] + l2[p];
      if (dd < d) d = dd;
    }
  });
  return d;
}

console.log(task1());
console.log(task2());
