const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => Number(/Generator . starts with (\d+)/.exec(line)[1]));

function task1() {
  let state = input;
  let c = 0;
  for (let i = 0; i < 40000000; i++) {
    state = state.map((a, i) => (a * (i === 0 ? 16807 : 48271)) % 2147483647);
    if ((state[0] & 0xffff) === (state[1] & 0xffff)) c++;
  }

  return c;
}

function task2() {
  let [a, b] = input;
  let c = 0;
  for (let i = 0; i < 5000000; i++) {
    do {
      a = (a * 16807) % 2147483647;
    } while (a % 4 !== 0);

    do {
      b = (b * 48271) % 2147483647;
    } while (b % 8 !== 0);

    if ((a & 0xffff) === (b & 0xffff)) c++;
  }

  return c;
}

console.log(task1());
console.log(task2());
