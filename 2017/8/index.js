const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => {
    const [
      _,
      reg,
      op,
      amount,
      c1,
      comp,
      c2,
    ] = /(\w+) (inc|dec) (-?\d+) if (-?\d+|\w+) (<|>|<=|>=|==|!=) (-?\d+|\w+)/.exec(line);
    return {
      reg,
      op,
      amount: Number(amount),
      c1,
      comp,
      c2,
    };
  });

function ex(line, state) {
  const c1 = /-?\d+/.exec(line.c1) ? Number(line.c1) : state[line.c1] ?? 0;
  const c2 = /-?\d+/.exec(line.c2) ? Number(line.c2) : state[line.c2] ?? 0;
  let cond;
  switch (line.comp) {
    case "<":
      cond = c1 < c2;
      break;
    case ">":
      cond = c1 > c2;
      break;
    case "<=":
      cond = c1 <= c2;
      break;
    case ">=":
      cond = c1 >= c2;
      break;
    case "==":
      cond = c1 === c2;
      break;
    case "!=":
      cond = c1 !== c2;
      break;
  }

  if (cond) {
    const newState = { ...state };
    if (newState[line.reg] === undefined) newState[line.reg] = 0;
    if (line.op === "inc") {
      newState[line.reg] += line.amount;
    } else {
      newState[line.reg] -= line.amount;
    }
    return newState;
  }

  return state;
}

function task1() {
  let state = {};
  input.forEach((line) => (state = ex(line, state)));
  return Object.values(state).reduce((a, b) => Math.max(a, b), -Infinity);
}

function task2() {
  let state = {};
  let max = -Infinity;
  input.forEach((line) => {
    state = ex(line, state);
    max = Object.values(state).reduce((a, b) => Math.max(a, b), max);
  });
  return max;
}

console.log(task1());
console.log(task2());
