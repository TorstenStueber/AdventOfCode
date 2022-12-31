const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function create() {
  const screen = [];
  for (let y = 0; y < 6; y++) {
    screen.push([]);
    for (let x = 0; x < 50; x++) {
      screen[y][x] = ".";
    }
  }
  input.forEach((line) => {
    const rect = /rect (\d+)x(\d+)/.exec(line);
    const row = /rotate row y=(\d+) by (\d+)/.exec(line);
    const col = /rotate column x=(\d+) by (\d+)/.exec(line);
    if (rect) {
      const [_, w, h] = rect;
      for (let y = 0; y < Number(h); y++) {
        for (let x = 0; x < Number(w); x++) {
          screen[y][x] = "#";
        }
      }
    } else if (row) {
      let [_, r, n] = row;
      r = Number(r);
      n = Number(n);
      while (n--) {
        const last = screen[r][49];
        for (let x = 49; x > 0; x--) {
          screen[r][x] = screen[r][x - 1];
        }
        screen[r][0] = last;
      }
    } else if (col) {
      let [_, c, n] = col;
      c = Number(c);
      n = Number(n);
      while (n--) {
        const last = screen[5][c];
        for (let y = 5; y > 0; y--) {
          screen[y][c] = screen[y - 1][c];
        }
        screen[0][c] = last;
      }
    }
  });

  return screen;
}

function task1() {
  const screen = create();

  let c = 0;
  for (let y = 0; y < 6; y++) {
    for (let x = 0; x < 50; x++) {
      if (screen[y][x] === "#") c++;
    }
  }

  return c;
}

function task2() {
  const screen = create();
  screen.forEach((line) => console.log(line.join("")));
}

console.log(task1());
console.log(task2());
