const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

const parsed = /(\d+) players; last marble is worth (\d+) points/.exec(input);
const players = parsed[1];
const maxMarble = parsed[2];

function task1() {
  const points = [];
  const marbles = [0];
  let current = 0;
  let player = 0;

  for (let i = 1; i <= maxMarble; i++) {
    if (i % 23 === 0) {
      current = (current - 7 + marbles.length) % marbles.length;
      if (points[player] === undefined) points[player] = 0;
      points[player] += i + marbles[current];
      marbles.splice(current, 1);
    } else {
      current = (current + 2) % marbles.length;
      marbles.splice(current, 0, i);
    }
    player = (player + 1) % players;
  }

  return Object.values(points).sort((a, b) => b - a)[0];
}

function task2() {
  const points = [];
  const marbles = [{ prev: 0, next: 0, value: 0 }];
  let current = 0;
  let player = 0;

  for (let i = 1; i <= maxMarble * 100; i++) {
    if (i % 23 === 0) {
      for (let l = 0; l < 7; l++) current = marbles[current].prev;
      if (points[player] === undefined) points[player] = 0;
      points[player] += i + marbles[current].value;

      marbles[marbles[current].next].prev = marbles[current].prev;
      marbles[marbles[current].prev].next = marbles[current].next;
      current = marbles[current].next;
    } else {
      current = marbles[current].next;
      marbles.push({ next: marbles[current].next, prev: current, value: i });
      marbles[marbles[current].next].prev = marbles.length - 1;
      marbles[current].next = marbles.length - 1;
      current = marbles.length - 1;
    }
    player = (player + 1) % players;
  }

  return Object.values(points).sort((a, b) => b - a)[0];
}

console.log(task1());
console.log(task2());
