const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

const numbers = [];
input.forEach((line, y) =>
  line.split("").forEach((a, x) => {
    if (/\d/.exec(a)) numbers[Number(a)] = [x, y];
  })
);

function findShortest(from, to) {
  function isAdmissibleState(oldStates, state) {
    if (oldStates.has(stringifyState(state))) return false;

    const { x, y } = state;
    return input[y][x] !== "#";
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
    return input[state.y][state.x] === to;
  }

  const todo = [];
  const oldStates = new Set();
  todo.push({ state: { x: numbers[from][0], y: numbers[from][1] }, steps: 0 });
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

const shortest = [];
for (let i = 0; i < numbers.length; i++) {
  for (let j = i + 1; j < numbers.length; j++) {
    const dist = findShortest(String(i), String(j));
    shortest[`${i},${j}`] = dist;
    shortest[`${j},${i}`] = dist;
  }
}

function task1() {
  let min = Infinity;

  function recurse(l, current, left) {
    if (left.size > 0) {
      Array.from(left).forEach((next) => {
        const newLeft = new Set(left);
        newLeft.delete(next);
        recurse(l + shortest[`${current},${next}`], next, newLeft);
      });
    } else {
      min = Math.min(min, l);
    }
  }

  const startingSet = new Set(Object.keys(numbers));
  startingSet.delete("0");
  recurse(0, "0", startingSet);

  return min;
}

function task2() {
  let min = Infinity;

  function recurse(l, current, left) {
    if (left.size > 0) {
      Array.from(left).forEach((next) => {
        const newLeft = new Set(left);
        newLeft.delete(next);
        recurse(l + shortest[`${current},${next}`], next, newLeft);
      });
    } else {
      min = Math.min(min, l + shortest[`${current},0`]);
    }
  }

  const startingSet = new Set(Object.keys(numbers));
  startingSet.delete("0");
  recurse(0, "0", startingSet);

  return min;
}

console.log(task1());
console.log(task2());
