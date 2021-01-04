const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => a.split(""));

function task1() {
  const size = input.length;
  let state = input;
  for (let i = 0; i < 100; i++) {
    let nextState = [];
    for (let y = 0; y < size; y++) {
      nextState.push([]);
      for (let x = 0; x < size; x++) {
        let c = 0;
        for (let yy = -1; yy <= 1; yy++) {
          for (let xx = -1; xx <= 1; xx++) {
            if (xx === 0 && yy === 0) continue;
            let xxx = x + xx;
            let yyy = y + yy;
            if (0 <= xxx && xxx < size && 0 <= yyy && yyy < size && state[yyy][xxx] === "#") c++;
          }
        }
        nextState[y][x] = (state[y][x] === "#" && (c === 2 || c === 3)) || (state[y][x] === "." && c === 3) ? "#" : ".";
      }
    }
    state = nextState;
  }

  let c = 0;
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      if (state[y][x] === "#") c++;
    }
  }
  return c;
}

function task2() {
  const size = input.length;
  let state = input;
  for (let i = 0; i < 100; i++) {
    let nextState = [];
    for (let y = 0; y < size; y++) {
      nextState.push([]);
      for (let x = 0; x < size; x++) {
        let c = 0;
        for (let yy = -1; yy <= 1; yy++) {
          for (let xx = -1; xx <= 1; xx++) {
            if (xx === 0 && yy === 0) continue;
            let xxx = x + xx;
            let yyy = y + yy;
            if (0 <= xxx && xxx < size && 0 <= yyy && yyy < size && state[yyy][xxx] === "#") c++;
          }
        }
        nextState[y][x] = (state[y][x] === "#" && (c === 2 || c === 3)) || (state[y][x] === "." && c === 3) ? "#" : ".";
      }
    }
    nextState[0][0] = "#";
    nextState[size - 1][0] = "#";
    nextState[0][size - 1] = "#";
    nextState[size - 1][size - 1] = "#";
    state = nextState;
  }

  let c = 0;
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      if (state[y][x] === "#") c++;
    }
  }
  return c;
}

console.log(task1());
console.log(task2());
