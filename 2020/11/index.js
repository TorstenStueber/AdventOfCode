const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => line.split(""));

function isOccupiedSeat(state, x, y) {
  return 0 <= y && y < state.length && 0 <= x && x < state[0].length && state[y][x] === "#";
}

function isOccupiedDir(state, x, y, xx, yy) {
  for (let c = 1; true; c++) {
    let xxx = x + c * xx;
    let yyy = y + c * yy;
    if (!(0 <= yyy && yyy < state.length && 0 <= xxx && xxx < state[0].length)) return false;
    if (state[yyy][xxx] === "#") return true;
    if (state[yyy][xxx] === "L") return false;
  }
}

function countOccupied(state, x, y, useDir) {
  let count = 0;
  [-1, 0, 1].forEach((xx) => {
    [-1, 0, 1].forEach((yy) => {
      if (xx === 0 && yy === 0) return;
      if (useDir) {
        if (isOccupiedDir(state, x, y, xx, yy)) count++;
      } else {
        if (isOccupiedSeat(state, x + xx, y + yy)) count++;
      }
    });
  });

  return count;
}

function iterateTask1(state) {
  let nextState = [];
  let hasChange = false;
  for (let y = 0; y < state.length; y++) {
    const line = [];
    for (let x = 0; x < state[y].length; x++) {
      if (state[y][x] === "L" && countOccupied(state, x, y, false) === 0) {
        line.push("#");
        hasChange = true;
      } else if (state[y][x] === "#" && countOccupied(state, x, y, false) >= 4) {
        line.push("L");
        hasChange = true;
      } else line.push(state[y][x]);
    }
    nextState.push(line);
  }

  return { nextState, hasChange };
}

function iterateTask2(state) {
  let nextState = [];
  let hasChange = false;
  for (let y = 0; y < state.length; y++) {
    const line = [];
    for (let x = 0; x < state[y].length; x++) {
      if (state[y][x] === "L" && countOccupied(state, x, y, true) === 0) {
        line.push("#");
        hasChange = true;
      } else if (state[y][x] === "#" && countOccupied(state, x, y, true) >= 5) {
        line.push("L");
        hasChange = true;
      } else line.push(state[y][x]);
    }
    nextState.push(line);
  }

  return { nextState, hasChange };
}

function task1() {
  let state = input;
  while (true) {
    const { nextState, hasChange } = iterateTask1(state);

    if (!hasChange) {
      let count = 0;
      for (let y = 0; y < state.length; y++) {
        for (let x = 0; x < state[y].length; x++) {
          if (state[y][x] === "#") count++;
        }
      }
      return count;
    }
    state = nextState;
  }
}

function task2() {
  let state = input;
  while (true) {
    const { nextState, hasChange } = iterateTask2(state);

    if (!hasChange) {
      let count = 0;
      for (let y = 0; y < state.length; y++) {
        for (let x = 0; x < state[y].length; x++) {
          if (state[y][x] === "#") count++;
        }
      }
      return count;
    }
    state = nextState;
  }
}

console.log(task1());
console.log(task2());
