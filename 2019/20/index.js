const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .split("\n")
  .map((line) => line.split(""));

const map = {};
input.forEach((line, y) => {
  line.forEach((a, x) => {
    map[`${x};${y}`] = a;
  });
});

const gates = {};

function addGate(name, x, y) {
  const isOuter = x === 2 || y === 2 || x === input[0].length - 3 || y === input.length - 3;
  if (gates[name] === undefined) gates[name] = {};
  if (isOuter) {
    gates[name].outer = [x, y];
  } else {
    gates[name].inner = [x, y];
  }
}

Object.entries(map).forEach(([c, a]) => {
  const [x, y] = c.split(";").map((z) => Number(z));
  if (/\w/.exec(a)) {
    if (map[`${x + 1};${y}`] !== undefined && /\w/.exec(map[`${x + 1};${y}`])) {
      const name = a + map[`${x + 1};${y}`];
      if (map[`${x - 1};${y}`] === ".") {
        addGate(name, x - 1, y);
      } else if (map[`${x + 2};${y}`] === ".") {
        addGate(name, x + 2, y);
      } else throw Error("Unexpected gate");
    }

    if (map[`${x};${y + 1}`] !== undefined && /\w/.exec(map[`${x};${y + 1}`])) {
      const name = a + map[`${x};${y + 1}`];
      if (map[`${x};${y - 1}`] === ".") {
        addGate(name, x, y - 1);
      } else if (map[`${x};${y + 2}`] === ".") {
        addGate(name, x, y + 2);
      } else throw Error("Unexpected gate");
    }
  }
});

const dirs = [
  [-1, 0],
  [1, 0],
  [0, -1],
  [0, 1],
];

function task1() {
  const todo = { 0: [gates["AA"].outer] };
  const seen = {};
  const [firstX, firstY] = todo[0][0];
  seen[`${firstX};${firstY}`] = 0;

  function add(x, y, d) {
    const key = `${x};${y}`;
    if (seen[key] !== undefined && seen[key] <= d) return;
    seen[key] = d;
    if (todo[d] === undefined) todo[d] = [];
    todo[d].push([x, y]);
  }

  while (true) {
    const d = Number(Object.keys(todo).sort((a, b) => a - b)[0]);
    const [x, y] = todo[d].shift();
    if (todo[d].length === 0) delete todo[d];

    const key = `${x};${y}`;
    if (seen[key] !== d) continue;
    if (x === gates["ZZ"].outer[0] && y === gates["ZZ"].outer[1]) return d;

    dirs.forEach(([xx, yy]) => {
      if (map[`${x + xx};${y + yy}`] === ".") add(x + xx, y + yy, d + 1);
    });
    Object.values(gates).forEach(({ outer, inner }) => {
      if (inner === undefined) return;
      if (inner[0] === x && inner[1] === y) add(outer[0], outer[1], d + 1);
      if (outer[0] === x && outer[1] === y) add(inner[0], inner[1], d + 1);
    });
  }
}

function task2() {
  const todo = { 0: [[...gates["AA"].outer, 0]] };
  const seen = {};
  const [firstX, firstY, firstLevel] = todo[0][0];
  seen[`${firstX};${firstY};${firstLevel}`] = 0;

  function add(x, y, l, d) {
    const key = `${x};${y};${l}`;
    if (seen[key] !== undefined && seen[key] <= d) return;
    seen[key] = d;
    if (todo[d] === undefined) todo[d] = [];
    todo[d].push([x, y, l]);
  }

  while (true) {
    const d = Number(Object.keys(todo).sort((a, b) => a - b)[0]);
    const [x, y, l] = todo[d].shift();
    if (todo[d].length === 0) delete todo[d];

    const key = `${x};${y};${l}`;
    if (seen[key] !== d) continue;
    if (l === 0 && x === gates["ZZ"].outer[0] && y === gates["ZZ"].outer[1]) return d;

    dirs.forEach(([xx, yy]) => {
      if (map[`${x + xx};${y + yy}`] === ".") add(x + xx, y + yy, l, d + 1);
    });
    Object.values(gates).forEach(({ outer, inner }) => {
      if (inner === undefined) return;
      if (inner[0] === x && inner[1] === y) add(outer[0], outer[1], l + 1, d + 1);
      if (outer[0] === x && outer[1] === y && l > 0) add(inner[0], inner[1], l - 1, d + 1);
    });
  }
}

console.log(task1());
console.log(task2());
