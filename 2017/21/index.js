const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .split("\n")
  .map((b) => b.split(" => ").map((x) => x.split("/")));

function findRule(state, size, startX, startY) {
  for (let rot = 0; rot < 4; rot++) {
    for (let flip = 0; flip < 2; flip++) {
      const oriented = [];
      for (let y = 0; y < size; y++) {
        oriented.push("");
        for (let x = 0; x < size; x++) {
          let xx = x;
          let yy = y;
          if (flip === 1) yy = size - y - 1;
          for (j = 0; j < rot; j++) {
            const c = xx;
            xx = yy;
            yy = size - c - 1;
          }
          oriented[y] += state[startY + yy][startX + xx];
        }
      }

      const foundRule = input.find(([r]) => {
        for (let y = 0; y < size; y++) {
          if (oriented[y] !== r[y]) return false;
        }
        return true;
      });
      if (foundRule) return foundRule;
    }
  }
}

function step(state) {
  let newState = [];
  if (state.length % 2 === 0) {
    for (let i = 0; i < (state.length / 2) * 3; i++) newState.push([]);

    for (let y = 0; y < state.length / 2; y++) {
      for (let x = 0; x < state.length / 2; x++) {
        const rule = findRule(state, 2, x * 2, y * 2);
        rule[1].forEach((line, yy) => {
          line.split("").forEach((a, xx) => {
            newState[y * 3 + yy][x * 3 + xx] = a;
          });
        });
      }
    }
  } else {
    for (let i = 0; i < (state.length / 3) * 4; i++) newState.push([]);

    for (let y = 0; y < state.length / 3; y++) {
      for (let x = 0; x < state.length / 3; x++) {
        const rule = findRule(state, 3, x * 3, y * 3);
        rule[1].forEach((line, yy) => {
          line.split("").forEach((a, xx) => {
            newState[y * 4 + yy][x * 4 + xx] = a;
          });
        });
      }
    }
  }

  return newState;
}

const startState = `.#.
..#
###`
  .split("\n")
  .map((line) => line.split(""));

function task1() {
  let state = startState;
  for (let i = 0; i < 5; i++) {
    state = step(state);
  }

  return state.reduce((a, b) => b.reduce((x, y) => x + (y === "#" ? 1 : 0), a), 0);
}

function task2() {
  let state = startState;
  for (let i = 0; i < 18; i++) {
    state = step(state);
  }

  return state.reduce((a, b) => b.reduce((x, y) => x + (y === "#" ? 1 : 0), a), 0);
}

console.log(task1());
console.log(task2());
