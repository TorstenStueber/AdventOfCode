const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => {
    const [_, first, second, length] = /(\w+) to (\w+) = (\d+)/.exec(line);
    return { first, second, length };
  });

const towns = Array.from(
  input.reduce((a, b) => {
    a.add(b.first);
    a.add(b.second);
    return a;
  }, new Set())
);

function task1() {
  let min = Infinity;

  function recurse(current, used, l) {
    if (used.length === towns.length - 1) {
      min = Math.min(min, l);
      return;
    }

    towns.forEach((t) => {
      if (current === t || used.includes(t)) return;
      const { length } = input.find(
        ({ first, second }) => (first === current && second === t) || (first === t && second === current)
      );
      recurse(t, [...used, current], l + Number(length));
    });
  }

  towns.forEach((t) => {
    recurse(t, [], 0);
  });

  return min;
}

function task2() {
  let max = 0;

  function recurse(current, used, l) {
    if (used.length === towns.length - 1) {
      max = Math.max(max, l);
      return;
    }

    towns.forEach((t) => {
      if (current === t || used.includes(t)) return;
      const { length } = input.find(
        ({ first, second }) => (first === current && second === t) || (first === t && second === current)
      );
      recurse(t, [...used, current], l + Number(length));
    });
  }

  towns.forEach((t) => {
    recurse(t, [], 0);
  });

  return max;
}

console.log(task1());
console.log(task2());
