const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split(",")
  .map((a) => Number(a));

function task1() {
  const spoken = {};
  let next, now;
  for (let i = 1; i <= 2020; i++) {
    if (i <= input.length) {
      now = input[i - 1];
    } else {
      now = next;
    }

    if (spoken[now] !== undefined) {
      next = i - spoken[now];
    } else {
      next = 0;
    }

    spoken[now] = i;
  }

  return now;
}

function task2() {
  const spoken = [];
  for (let i = 1; i <= 30000000; i++) {
    spoken.push([]);
  }

  let next, now;
  for (let i = 1; i <= 30000000; i++) {
    if (i <= input.length) {
      now = input[i - 1];
    } else {
      now = next;
    }

    const nowSpoken = spoken[now];
    if (nowSpoken.length >= 1) {
      next = i - nowSpoken[nowSpoken.length - 1];
    } else {
      next = 0;
    }
    nowSpoken.push(i);

    if (!(i % 1000000)) console.log(i);
  }

  return now;
}

console.log(task1());
console.log(task2());
