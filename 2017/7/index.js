const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => {
    const [first, last] = line.split(" -> ");
    const [_, name, weight] = /(\w+) \((\d+)\)/.exec(first);
    return { name, weight: Number(weight), sub: last?.split(", ") ?? [] };
  });

function task1() {
  const subs = new Set();
  input.forEach(({ sub }) => sub.forEach((a) => subs.add(a)));
  return input.find(({ name }) => !subs.has(name)).name;
}

function task2() {
  const weights = {};
  while (Object.keys(weights).length < input.length) {
    input.forEach(({ name, weight, sub }) => {
      if (weights[name] === undefined && sub.every((s) => weights[s] !== undefined)) {
        weights[name] = weight + sub.reduce((a, b) => a + weights[b], 0);
      }
    });
  }

  function findOdd(n) {
    const s = input.find(({ name }) => n === name).sub;
    if (s.length <= 2) return undefined;
    let r = weights[s[0]] === weights[s[1]] ? weights[s[0]] : weights[s[2]];
    const nn = s.find((sn) => r !== weights[sn]);
    if (nn === undefined) return undefined;
    return { name: nn, expected: r };
  }

  let current = task1();
  let expected;
  while (true) {
    const odd = findOdd(current);
    if (odd) {
      current = odd.name;
      expected = odd.expected;
    } else {
      return input.find(({ name }) => name === current).weight + expected - weights[current];
    }
  }
}

console.log(task1());
console.log(task2());
