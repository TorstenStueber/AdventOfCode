const { readFileSync, stat } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => {
    const chips = new Set();
    const gens = new Set();
    const [_, s] = /The \w+ floor contains (.+)\.$/.exec(line);
    const parts = s === "nothing relevant" ? [] : s.split(/, | and /);
    parts.map((part) => {
      const chip = /a (\w+)-compatible microchip/.exec(part);
      const gen = /a (\w+) generator/.exec(part);
      if (chip) {
        chips.add(chip[1]);
      } else {
        gens.add(gen[1]);
      }
    });
    return { chips, gens };
  });

function isAdmissibleState(oldStates, state) {
  if (oldStates.has(stringifyState(state))) return false;

  return state.floors.every((floor) => {
    if (floor.gens.size === 0) return true;
    return Array.from(floor.chips).every((chip) => {
      return floor.gens.has(chip);
    });
  });
}

function orderSet(set) {
  return Array.from(set)
    .sort((a, b) => (a < b ? -1 : 1))
    .join(",");
}

function stringifyState(state) {
  return `${state.level}:${state.floors.map((floor) => `${orderSet(floor.gens)}-${orderSet(floor.chips)}`).join(":")}`;
}

function generateFollowUpState(oldStates, state) {
  const nextStates = [];
  function genWithLevel(level) {
    const gens = Array.from(state.floors[state.level].gens);
    const chips = Array.from(state.floors[state.level].chips);

    function genState() {
      const newState = { level, floors: [...state.floors] };
      newState.floors[state.level] = {
        gens: new Set(state.floors[state.level].gens),
        chips: new Set(state.floors[state.level].chips),
      };

      newState.floors[level] = {
        gens: new Set(state.floors[level].gens),
        chips: new Set(state.floors[level].chips),
      };
      return newState;
    }

    function move(newState, i) {
      if (i < gens.length) {
        newState.floors[state.level].gens.delete(gens[i]);
        newState.floors[level].gens.add(gens[i]);
      } else {
        newState.floors[state.level].chips.delete(chips[i - gens.length]);
        newState.floors[level].chips.add(chips[i - gens.length]);
      }
    }

    for (let i = 0; i < gens.length + chips.length; i++) {
      const newState = genState();
      move(newState, i);
      if (isAdmissibleState(oldStates, newState)) nextStates.push(newState);
    }

    for (let i = 0; i < gens.length + chips.length; i++) {
      for (let j = i + 1; j < gens.length + chips.length; j++) {
        const newState = genState();
        move(newState, i);
        move(newState, j);
        if (isAdmissibleState(oldStates, newState)) nextStates.push(newState);
      }
    }
  }

  if (state.level > 0) {
    genWithLevel(state.level - 1);
  }
  if (state.level < 3) {
    genWithLevel(state.level + 1);
  }

  return nextStates;
}

function isCompleteState(state) {
  return (
    state.level === 3 && state.floors.slice(0, 3).every((floor) => floor.chips.size === 0 && floor.gens.size === 0)
  );
}

function task1() {
  const todo = [];
  const oldStates = new Set();
  todo.push({ state: { level: 0, floors: input }, steps: 0 });
  oldStates.add(stringifyState(todo[0].state));

  let lastSteps = 0;
  do {
    const { state, steps } = todo.shift();
    if (steps !== lastSteps) {
      lastSteps = steps;
      console.log(">", steps);
    }

    if (isCompleteState(state)) return steps;
    generateFollowUpState(oldStates, state).forEach((nextState) => {
      oldStates.add(stringifyState(nextState));
      todo.push({ state: nextState, steps: steps + 1 });
    });
  } while (true);
}

function task2() {
  const todo = [];
  const oldStates = new Set();
  const input2 = [...input];
  input2[0] = { gens: new Set(input2[0].gens), chips: new Set(input2[0].chips) };
  input2[0].gens.add("elerium");
  input2[0].gens.add("dilithium");
  input2[0].chips.add("elerium");
  input2[0].chips.add("dilithium");

  todo.push({ state: { level: 0, floors: input2 }, steps: 0 });
  oldStates.add(stringifyState(todo[0].state));

  let lastSteps = 0;
  do {
    const { state, steps } = todo.shift();
    if (steps !== lastSteps) {
      lastSteps = steps;
      console.log(">", steps);
    }

    if (isCompleteState(state)) return steps;
    generateFollowUpState(oldStates, state).forEach((nextState) => {
      oldStates.add(stringifyState(nextState));
      todo.push({ state: nextState, steps: steps + 1 });
    });
  } while (true);
}

console.log(task1());
console.log(task2());
