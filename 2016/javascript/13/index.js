const { readFileSync } = require("fs");
const { join } = require("path");

const input = Number(readFileSync(join(__dirname, "input.txt"), "utf-8").trim());

function isAdmissibleState(oldStates, state) {
  if (oldStates.has(stringifyState(state))) return false;

  const { x, y } = state;
  if (x < 0 || y < 0) return false;
  let number = x * x + 3 * x + 2 * x * y + y + y * y + input;
  let c = 0;
  while (number > 0) {
    if (number & 1) c++;
    number >>= 1;
  }
  return c % 2 == 0;
}

function stringifyState(state) {
  return `${state.x}:${state.y}`;
}

const dirs = [
  [-1, 0],
  [1, 0],
  [0, 1],
  [0, -1],
];

function generateFollowUpState(oldStates, state) {
  const nextStates = [];

  const { x, y } = state;
  dirs.forEach(([xx, yy]) => {
    const newState = { x: x + xx, y: y + yy };
    if (isAdmissibleState(oldStates, newState)) nextStates.push(newState);
  });

  return nextStates;
}

function isCompleteState(state) {
  return state.x === 31 && state.y === 39;
}

function task1() {
  const todo = [];
  const oldStates = new Set();
  todo.push({ state: { x: 1, y: 1 }, steps: 0 });
  oldStates.add(stringifyState(todo[0].state));

  do {
    const { state, steps } = todo.shift();

    if (isCompleteState(state)) return steps;
    generateFollowUpState(oldStates, state).forEach((nextState) => {
      oldStates.add(stringifyState(nextState));
      todo.push({ state: nextState, steps: steps + 1 });
    });
  } while (todo.length);
}

function task2() {
  const todo = [];
  const oldStates = new Set();
  todo.push({ state: { x: 1, y: 1 }, steps: 0 });
  oldStates.add(stringifyState(todo[0].state));

  do {
    const { state, steps } = todo.shift();
    if (steps >= 50) break;

    if (isCompleteState(state)) return steps;
    generateFollowUpState(oldStates, state).forEach((nextState) => {
      oldStates.add(stringifyState(nextState));
      todo.push({ state: nextState, steps: steps + 1 });
    });
  } while (todo.length);

  return oldStates.size;
}

console.log(task1());
console.log(task2());
