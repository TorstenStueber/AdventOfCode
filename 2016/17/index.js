const { readFileSync } = require("fs");
const { join } = require("path");
const { createHash } = require("crypto");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

function isAdmissibleState(oldStates, state) {
  if (oldStates.has(stringifyState(state))) return false;

  return state.x >= 0 && state.y >= 0 && state.x <= 3 && state.y <= 3;
}

function stringifyState(state) {
  return state.seq;
}

const dirs = {
  U: [0, -1],
  D: [0, 1],
  L: [-1, 0],
  R: [1, 0],
};

function generateFollowUpState(oldStates, state) {
  const nextStates = [];

  const { x, y, seq } = state;
  hash = createHash("MD5");
  hash.update(`${input}${state.seq}`);
  const digest = hash.digest("hex");
  Object.entries(dirs).forEach(([d, [xx, yy]], i) => {
    if ("bcdef".indexOf(digest[i]) === -1) return;
    const newState = { x: x + xx, y: y + yy, seq: seq + d };
    if (isAdmissibleState(oldStates, newState)) nextStates.push(newState);
  });

  return nextStates;
}

function isCompleteState(state) {
  return state.x === 3 && state.y === 3;
}

function task1() {
  const todo = [];
  const oldStates = new Set();

  todo.push({ state: { x: 0, y: 0, seq: "" }, steps: 0 });
  oldStates.add(stringifyState(todo[0].state));

  do {
    const { state, steps } = todo.shift();

    if (isCompleteState(state)) return state.seq;
    generateFollowUpState(oldStates, state).forEach((nextState) => {
      oldStates.add(stringifyState(nextState));
      todo.push({ state: nextState, steps: steps + 1 });
    });
  } while (todo.length);
}

function task2() {
  let maxLength = 0;
  const todo = [];
  const oldStates = new Set();

  todo.push({ state: { x: 0, y: 0, seq: "" }, steps: 0 });
  oldStates.add(stringifyState(todo[0].state));

  do {
    const { state, steps } = todo.shift();

    if (isCompleteState(state)) {
      maxLength = Math.max(maxLength, state.seq.length);
    } else {
      generateFollowUpState(oldStates, state).forEach((nextState) => {
        oldStates.add(stringifyState(nextState));
        todo.push({ state: nextState, steps: steps + 1 });
      });
    }
  } while (todo.length);

  return maxLength;
}

console.log(task1());
console.log(task2());
