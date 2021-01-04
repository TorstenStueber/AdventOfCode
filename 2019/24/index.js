const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => line.split(""));

function task1() {
  let state = input;
  let seen = new Set();

  while (true) {
    let key = state.map((line) => line.join("")).join(";");
    if (seen.has(key)) {
      return parseInt(
        state
          .map((line) => line.join(""))
          .join("")
          .split("")
          .reverse()
          .join("")
          .replace(/\./g, "0")
          .replace(/#/g, "1"),
        2
      );
    }
    seen.add(key);

    let nextState = [];
    for (let y = 0; y < 5; y++) {
      nextState.push([]);
      for (let x = 0; x < 5; x++) {
        c = 0;
        if (x > 0 && state[y][x - 1] === "#") c++;
        if (x < 4 && state[y][x + 1] === "#") c++;
        if (y > 0 && state[y - 1][x] === "#") c++;
        if (y < 4 && state[y + 1][x] === "#") c++;
        nextState[y][x] = (state[y][x] === "#" && c === 1) || (state[y][x] === "." && (c === 1 || c === 2)) ? "#" : ".";
      }
    }
    state = nextState;
  }
}

function task2() {
  const neighbors = {};
  for (let x = 0; x < 5; x++) {
    for (let y = 0; y < 5; y++) {
      if (x === 2 && y === 2) continue;
      const n = [];

      if (x > 0) {
        if (x !== 3 || y !== 2) {
          n.push([-1, 0, 0]);
        } else {
          for (let i = -2; i <= 2; i++) n.push([1, i, 1]);
        }
      } else {
        n.push([1, 2 - y, -1]);
      }

      if (x < 4) {
        if (x !== 1 || y !== 2) {
          n.push([1, 0, 0]);
        } else {
          for (let i = -2; i <= 2; i++) n.push([-1, i, 1]);
        }
      } else {
        n.push([-1, 2 - y, -1]);
      }

      if (y > 0) {
        if (y !== 3 || x !== 2) {
          n.push([0, -1, 0]);
        } else {
          for (let i = -2; i <= 2; i++) n.push([i, 1, 1]);
        }
      } else {
        n.push([2 - x, 1, -1]);
      }

      if (y < 4) {
        if (y !== 1 || x !== 2) {
          n.push([0, 1, 0]);
        } else {
          for (let i = -2; i <= 2; i++) n.push([i, -1, 1]);
        }
      } else {
        n.push([2 - x, -1, -1]);
      }
      neighbors[`${x};${y}`] = n;
    }
  }

  let bugs = new Set();
  input.forEach((line, y) =>
    line.forEach((a, x) => {
      if (a === "#") bugs.add(`${x};${y};${0}`);
    })
  );
  let minLevel = 0;
  let maxLevel = 0;

  for (let i = 0; i < 200; i++) {
    let nextBugs = new Set();

    for (let level = minLevel - 1; level <= maxLevel + 1; level++) {
      for (let x = 0; x < 5; x++) {
        for (let y = 0; y < 5; y++) {
          if (x === 2 && y === 2) continue;

          let c = 0;
          neighbors[`${x};${y}`].forEach(([xx, yy, ll]) => {
            if (bugs.has(`${x + xx};${y + yy};${level + ll}`)) c++;
          });

          if (c === 1 || (!bugs.has(`${x};${y};${level}`) && c === 2)) {
            minLevel = Math.min(minLevel, level);
            maxLevel = Math.max(maxLevel, level);
            nextBugs.add(`${x};${y};${level}`);
          }
        }
      }
    }

    bugs = nextBugs;
  }

  return bugs.size;
}

console.log(task1());
console.log(task2());
