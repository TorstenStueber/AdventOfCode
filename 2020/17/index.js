const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => a.split(""));

function step(state, min, max) {
  let nextState = new Set();

  for (x = min; x <= max; x++)
    for (y = min; y <= max; y++)
      for (z = min; z <= max; z++) {
        let count = 0;
        for (xx = -1; xx <= 1; xx++)
          for (yy = -1; yy <= 1; yy++)
            for (zz = -1; zz <= 1; zz++) {
              if (xx !== 0 || yy !== 0 || zz !== 0) {
                const coord = `${x + xx};${y + yy};${z + zz}`;
                if (state.has(coord)) count++;
              }
            }

        const coord = `${x};${y};${z}`;
        const active = state.has(coord);
        if ((active && (count === 2 || count === 3)) || (!active && count === 3)) {
          nextState.add(coord);
        }
      }

  return nextState;
}

function step4(state, min, max) {
  let nextState = new Set();
  for (x = min; x <= max; x++)
    for (y = min; y <= max; y++)
      for (z = min; z <= max; z++)
        for (w = min; w <= max; w++) {
          let count = 0;
          for (xx = -1; xx <= 1; xx++)
            for (yy = -1; yy <= 1; yy++)
              for (zz = -1; zz <= 1; zz++)
                for (ww = -1; ww <= 1; ww++) {
                  if (xx !== 0 || yy !== 0 || zz !== 0 || ww !== 0) {
                    const coord = `${x + xx};${y + yy};${z + zz};${w + ww}`;
                    if (state.has(coord)) count++;
                  }
                }
          const coord = `${x};${y};${z};${w}`;
          const active = state.has(coord);
          if ((active && (count === 2 || count === 3)) || (!active && count === 3)) {
            nextState.add(coord);
          }
        }
  return nextState;
}

function task1() {
  let state = new Set();
  for (let y = 0; y < input.length; y++)
    for (let x = 0; x < input[0].length; x++) {
      if (input[y][x] === "#") state.add(`${x};${y};0`);
    }

  for (let i = 0; i < 6; i++) {
    state = step(state, -i - 1, i + 1 + input.length);
  }

  return state.size;
}

function task2() {
  let state = new Set();
  for (let y = 0; y < input.length; y++)
    for (let x = 0; x < input[0].length; x++) {
      if (input[y][x] === "#") state.add(`${x};${y};0;0`);
    }

  for (let i = 0; i < 6; i++) {
    state = step4(state, -i - 1, i + 1 + input.length);
  }

  return state.size;
}

console.log(task1());
console.log(task2());
