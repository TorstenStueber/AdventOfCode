const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => line.split(""));

const positions = {};
input.forEach((line, y) => {
  line.forEach((a, x) => {
    if (a !== "." && a !== "#") {
      positions[a] = [x, y];
    }
  });
});

const keys = Object.keys(positions).reduce((a, b) => {
  if (b !== "@") a.add(b.toLowerCase());
  return a;
}, new Set());

const dirs = [
  [-1, 0],
  [1, 0],
  [0, -1],
  [0, 1],
];

function findKeys(x, y, foundKeys, map) {
  const accessed = { [`${x};${y}`]: 0 };
  const todo = [[x, y]];
  const keys = [];

  while (todo.length > 0) {
    const [x, y] = todo.shift();

    dirs.forEach(([incX, incY]) => {
      const xx = x + incX;
      const yy = y + incY;
      if (0 <= yy && yy < map.length && 0 <= xx && xx < map[yy].length && map[yy][xx] !== "#") {
        const key = `${xx};${yy}`;
        if (accessed[key] !== undefined) return;
        accessed[key] = accessed[`${x};${y}`] + 1;
        const s = map[yy][xx];

        if (s !== "." && s !== "@" && s.toLowerCase() === s) {
          if (!foundKeys.has(s)) {
            keys.push({ key: s, steps: accessed[key] });
            return;
          }
        }

        if (s !== "." && s !== "@" && s.toUpperCase() === s) {
          if (!foundKeys.has(s.toLowerCase())) return;
        }

        todo.push([xx, yy]);
      }
    });
  }

  return keys;
}

function task1() {
  const todo = { 0: [[new Set(), "@"]] };
  const seen = {};
  seen[`;@`] = 0;

  while (true) {
    const d = Number(Object.keys(todo).sort((a, b) => a - b)[0]);
    const [s, item] = todo[d].shift();
    if (todo[d].length === 0) delete todo[d];

    const key = `${Array.from(s).sort()};${item}`;
    if (seen[key] !== d) continue;
    if (s.size === keys.size) return d;

    const nextKeys = findKeys(positions[item][0], positions[item][1], s, input);
    nextKeys.forEach(({ key, steps }) => {
      const ss = new Set(s);
      const dd = d + steps;
      ss.add(key);
      const k = `${Array.from(ss).sort()};${key}`;
      if (seen[k] !== undefined && seen[k] <= dd) return;
      seen[k] = dd;
      if (todo[dd] === undefined) todo[dd] = [];
      todo[dd].push([ss, key]);
    });
  }
}

function task2() {
  const map2 = [...input.map((line) => [...line])];
  map2[positions["@"][1]][positions["@"][0]] = "#";
  map2[positions["@"][1] - 1][positions["@"][0]] = "#";
  map2[positions["@"][1] + 1][positions["@"][0]] = "#";
  map2[positions["@"][1]][positions["@"][0] - 1] = "#";
  map2[positions["@"][1]][positions["@"][0] + 1] = "#";

  const todo = { 0: [[new Set(), "@", "@", "@", "@"]] };
  const seen = {};
  seen[`;@;@;@;@`] = 0;

  while (true) {
    const d = Number(Object.keys(todo).sort((a, b) => a - b)[0]);
    const [s, item1, item2, item3, item4] = todo[d].shift();
    if (todo[d].length === 0) delete todo[d];

    const key = `${Array.from(s).sort()};${item1};${item2};${item3};${item4}`;
    if (seen[key] !== d) continue;
    if (s.size === keys.size) return d;

    [item1, item2, item3, item4].forEach((item, i) => {
      let x = positions[item][0];
      let y = positions[item][1];
      if (item === "@") {
        switch (i) {
          case 0:
            x--;
            y--;
            break;
          case 1:
            x++;
            y--;
            break;
          case 2:
            x--;
            y++;
            break;
          case 3:
            x++;
            y++;
            break;
        }
      }

      const nextKeys = findKeys(x, y, s, map2);
      nextKeys.forEach(({ key, steps }) => {
        let it1 = item1;
        let it2 = item2;
        let it3 = item3;
        let it4 = item4;
        switch (i) {
          case 0:
            it1 = key;
            break;
          case 1:
            it2 = key;
            break;
          case 2:
            it3 = key;
            break;
          case 3:
            it4 = key;
            break;
        }
        const ss = new Set(s);
        const dd = d + steps;
        ss.add(key);
        const k = `${Array.from(ss).sort()};${it1};${it2};${it3};${it4}`;
        if (seen[k] !== undefined && seen[k] <= dd) return;
        seen[k] = dd;
        if (todo[dd] === undefined) todo[dd] = [];
        todo[dd].push([ss, it1, it2, it3, it4]);
      });
    });
  }
}

console.log(task1());
console.log(task2());
