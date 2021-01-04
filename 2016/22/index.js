const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .slice(2)
  .map((line) => {
    const [
      _,
      x,
      y,
      size,
      used,
      avail,
      perc,
    ] = /\/dev\/grid\/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%/.exec(line);
    return {
      x: Number(x),
      y: Number(y),
      size: Number(size),
      used: Number(used),
      avail: Number(avail),
      perc: Number(perc),
    };
  });

const dirs = [
  [-1, 0],
  [1, 0],
  [0, -1],
  [0, 1],
];

let maxX = 0;
let maxY = 0;
let maxAvail = 0;
let minUsed = Infinity;
let maxUsed = 0;
let minSize = Infinity;
input.forEach(({ x, y, avail, used, size }) => {
  maxX = Math.max(maxX, x);
  maxY = Math.max(maxY, y);
  minSize = Math.min(minSize, size);
  if (size < 100) {
    maxUsed = Math.max(maxUsed, used);
  }
  if (used > 0) {
    maxAvail = Math.max(maxAvail, avail);
    minUsed = Math.min(minUsed, used);
  }
});

const grid = [];
input.forEach((line) => {
  const { x, y } = line;
  if (grid[y] === undefined) grid[y] = [];
  grid[y][x] = line;
});

function task1() {
  let c = 0;
  input.forEach((line1, i) => {
    input.forEach((line2, j) => {
      if (i === j) return;
      if (line1.used !== 0 && line1.used <= line2.avail) c++;
    });
  });
  return c;
}

function isAdmissibleState(oldStates, state) {
  if (oldStates.has(stringifyState(state))) return false;

  const { bubbleX, bubbleY } = state;
  if (bubbleX < 0 || bubbleY < 0 || bubbleX > maxX || bubbleY > maxY) return false;
  if (grid[bubbleY][bubbleX].size > 100) return false;
  return true;
}

function stringifyState(state) {
  return `${state.bubbleX}:${state.bubbleY}:${state.dataX}:${state.dataY}`;
}

function generateFollowUpState(oldStates, state) {
  const nextStates = [];

  const { bubbleX, bubbleY, dataX, dataY } = state;
  dirs.forEach(([xx, yy]) => {
    const newState = { dataX, dataY, bubbleX: bubbleX + xx, bubbleY: bubbleY + yy };
    if (newState.bubbleX === dataX && newState.bubbleY === dataY) {
      newState.dataX = bubbleX;
      newState.dataY = bubbleY;
    }
    if (isAdmissibleState(oldStates, newState)) nextStates.push(newState);
  });

  return nextStates;
}

function isCompleteState(state) {
  return state.dataX === 0 && state.dataY === 0;
}

function task2() {
  const todo = [];
  const oldStates = new Set();
  const { x: bubbleX, y: bubbleY } = input.find((line) => line.used === 0);
  todo.push({ state: { bubbleX, bubbleY, dataX: maxX, dataY: 0 }, steps: 0 });
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

console.log(task1());
console.log(task2());
