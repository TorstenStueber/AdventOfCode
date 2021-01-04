const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("");

const dirs = { E: [1, 0], W: [-1, 0], S: [0, 1], N: [0, -1] };

function findShortest() {
  const accessible = new Set();

  function merge(a, b) {
    b.forEach(([x, y]) => a.add(`${x};${y}`));
  }

  function recurse(start, positions) {
    let currentPositions = [...positions.map((a) => [...a])];
    let targetPositions = new Set();
    while (start < input.length) {
      let s = input[start];
      if (s === ")" || s === "$") {
        start++;
        break;
      } else if (s === "(") {
        const result = recurse(start + 1, currentPositions);
        currentPositions = Array.from(result.targetPositions).map((x) => x.split(";").map((a) => Number(a)));
        start = result.start;
      } else if (s === "|") {
        start++;
        merge(targetPositions, currentPositions);
        currentPositions = [...positions.map((a) => [...a])];
      } else {
        for (let i = 0; i < currentPositions.length; i++) {
          currentPositions[i][0] += dirs[s][0];
          currentPositions[i][1] += dirs[s][1];
          accessible.add(`${currentPositions[i][0]};${currentPositions[i][1]}`);
          currentPositions[i][0] += dirs[s][0];
          currentPositions[i][1] += dirs[s][1];
          accessible.add(`${currentPositions[i][0]};${currentPositions[i][1]}`);
        }
        start++;
      }
    }

    merge(targetPositions, currentPositions);
    return { targetPositions, start };
  }

  accessible.add(`0;0`);
  recurse(1, [[0, 0]]);

  const todo = [[0, 0]];
  const results = {};
  results[`${0};${0}`] = 0;

  while (todo.length) {
    const [x, y] = todo.shift();
    const d = results[`${x};${y}`];
    Object.values(dirs).forEach(([xx, yy]) => {
      const key = `${x + xx};${y + yy}`;
      if (accessible.has(key)) {
        if (results[key] === undefined) {
          todo.push([x + xx, y + yy]);
          results[key] = d + 1;
        }
      }
    });
  }

  return results;
}

function task1() {
  const results = findShortest();
  return Object.values(results).sort((a, b) => b - a)[0] / 2;
}

function task2() {
  const results = findShortest();
  let c = 0;
  Object.values(results).forEach((v) => {
    if (v % 2 === 0 && v >= 2000) c++;
  });
  return c;
}

console.log(task1());
console.log(task2());
