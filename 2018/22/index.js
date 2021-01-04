const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

const depth = Number(input[0].split(": ")[1]);
const target = input[1]
  .split(": ")[1]
  .split(",")
  .map((a) => Number(a));

const geoIndex = {};

function getGeoIndex(x, y) {
  const key = `${x};${y}`;
  if (geoIndex[key] === undefined) {
    if (x === 0 && y === 0) {
      geoIndex[key] = 0;
    } else if (x === target[0] && y === target[1]) {
      geoIndex[key] = 0;
    } else if (y === 0) {
      geoIndex[key] = x * 16807;
    } else if (x === 0) {
      geoIndex[key] = y * 48271;
    } else {
      geoIndex[key] = getErosionLevel(x - 1, y) * getErosionLevel(x, y - 1);
    }
  }
  return geoIndex[key];
}

function getErosionLevel(x, y) {
  return (getGeoIndex(x, y) + depth) % 20183;
}

function task1() {
  let s = 0;
  for (let y = 0; y <= target[1]; y++) {
    for (let x = 0; x <= target[0]; x++) {
      s += getErosionLevel(x, y) % 3;
    }
  }
  return s;
}

function compatible(x, y, item) {
  const type = getErosionLevel(x, y) % 3;
  switch (type) {
    case 0:
      return item === "climbing" || item === "torch";
    case 1:
      return item === "climbing" || item === "neither";
    case 2:
      return item === "neither" || item === "torch";
  }
}

function task2() {
  const todo = { 0: [[0, 0, "torch"]] };
  const seen = {};
  seen[`0;0;torch`] = "";

  function add(x, y, item, d) {
    const key = `${x};${y};${item}`;
    if (seen[key] !== undefined) {
      if (seen[key] <= d) return;
      todo[seen[key]] = todo[seen[key]].filter(([xx, yy, ii]) => xx !== x || yy !== y || ii !== item);
      if (todo[seen[key]].length === 0) delete todo[seen[key]];
    }
    seen[key] = d;
    if (todo[d] === undefined) todo[d] = [];
    todo[d].push([x, y, item]);
  }

  while (true) {
    const d = Number(Object.keys(todo).sort((a, b) => a - b)[0]);
    const [x, y, item] = todo[d].shift();
    const key = `${x};${y};${item}`;
    const b = seen[key];

    if (x === target[0] && y === target[1] && item === "torch") return d;
    if (todo[d].length === 0) delete todo[d];

    if (x > 0 && compatible(x - 1, y, item)) add(x - 1, y, item, d + 1, b);
    if (y > 0 && compatible(x, y - 1, item)) add(x, y - 1, item, d + 1, b);
    if (compatible(x + 1, y, item)) add(x + 1, y, item, d + 1, b);
    if (compatible(x, y + 1, item)) add(x, y + 1, item, d + 1, b);

    if (item !== "climbing" && compatible(x, y, "climbing")) add(x, y, "climbing", d + 7, b);
    if (item !== "torch" && compatible(x, y, "torch")) add(x, y, "torch", d + 7, b);
    if (item !== "neither" && compatible(x, y, "neither")) add(x, y, "neither", d + 7, b);
  }
}

console.log(task1());
console.log(task2());
