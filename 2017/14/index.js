const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

function knotHash(string) {
  const processedInput = [...string.split("").map((a) => a.charCodeAt(0)), 17, 31, 73, 47, 23];
  let list = [];
  for (let i = 0; i < 256; i++) list.push(i);
  let current = 0;
  let skip = 0;
  for (rounds = 0; rounds < 64; rounds++) {
    processedInput.forEach((length) => {
      const sublist = [...list, ...list].slice(current, current + length).reverse();
      for (let i = 0; i < length; i++) {
        list[(i + current) % 256] = sublist[i];
      }
      current = (current + length + skip) % 256;
      skip++;
    });
  }

  let result = "";
  for (p = 0; p < 16; p++) {
    let xor = 0;
    for (j = 0; j < 16; j++) {
      xor ^= list[p * 16 + j];
    }
    result += xor.toString(16).padStart(2, "0");
  }

  return result;
}

function grid() {
  let grid = new Set();
  for (let i = 0; i < 128; i++) {
    const h = knotHash(`${input}-${i}`);
    h.split("").forEach((a, j) => {
      let n = parseInt(a, 16);
      let k = 3;
      while (n > 0) {
        if (n & 1) {
          grid.add(`${j * 4 + k};${i}`);
        }
        k--;
        n = Math.floor(n / 2);
      }
    });
  }

  return grid;
}

function task1() {
  return grid().size;
}

const dirs = [
  [1, 0],
  [-1, 0],
  [0, 1],
  [0, -1],
];

function task2() {
  const g = grid();
  let areas = 0;
  while (g.size > 0) {
    const todo = [Array.from(g)[0]];
    areas++;
    g.delete(todo[0]);

    while (todo.length > 0) {
      const [x, y] = todo.shift().split(";");
      dirs.forEach(([xx, yy]) => {
        const key = `${Number(x) + xx};${Number(y) + yy}`;
        if (g.has(key)) {
          g.delete(key);
          todo.push(key);
        }
      });
    }
  }

  return areas;
}

console.log(task1());
console.log(task2());
