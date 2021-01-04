const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function step(state) {
  const newState = [];
  for (let y = 0; y < state.length; y++) {
    newState.push([]);
    for (let x = 0; x < state[y].length; x++) {
      let trees = 0;
      let lyards = 0;
      for (let xx = -1; xx <= 1; xx++) {
        for (let yy = -1; yy <= 1; yy++) {
          if (xx === 0 && yy === 0) continue;
          if (x + xx < 0 || x + xx >= state[y].length || y + yy < 0 || y + yy >= state.length) continue;
          if (state[y + yy][x + xx] === "|") trees++;
          if (state[y + yy][x + xx] === "#") lyards++;
        }
      }
      if (state[y][x] === ".") {
        newState[y][x] = trees >= 3 ? "|" : ".";
      }
      if (state[y][x] === "|") {
        newState[y][x] = lyards >= 3 ? "#" : "|";
      }
      if (state[y][x] === "#") {
        newState[y][x] = lyards >= 1 && trees >= 1 ? "#" : ".";
      }
    }
  }
  return newState;
}

function task1() {
  let state = [...input];
  for (i = 0; i < 10; i++) {
    state = step(state);
  }

  let trees = 0;
  let lyards = 0;
  state.forEach((line) =>
    line.forEach((a) => {
      if (a === "|") trees++;
      if (a === "#") lyards++;
    })
  );

  return trees * lyards;
}

function task2() {
  let state = [...input.map((a) => a.split(""))];
  const limit = 1000000000;
  const seen = {};

  for (i = 0; i < 1000000000; i++) {
    const key = state.map((a) => a.join("")).join(";");
    if (seen[key] !== undefined) {
      const index = ((limit - seen[key]) % (i - seen[key])) + seen[key];
      goalKey = Object.keys(seen).find((key) => seen[key] === index);
      state = goalKey.split(";").map((a) => a.split(""));
      break;
    }
    seen[key] = i;
    state = step(state);
  }

  let trees = 0;
  let lyards = 0;
  state.forEach((line) =>
    line.forEach((a) => {
      if (a === "|") trees++;
      if (a === "#") lyards++;
    })
  );

  return trees * lyards;
}

console.log(task1());
console.log(task2());
