const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) =>
    line
      .split(" <-> ")[1]
      .split(", ")
      .map((a) => Number(a))
  );

function groupOf(n) {
  const s = new Set();
  s.add(n);
  let added;
  do {
    added = false;
    input.forEach((r, i) => {
      if (!s.has(i) && r.some((a) => s.has(a))) {
        s.add(i);
        added = true;
      }
    });
  } while (added);

  return s;
}

function task1() {
  return groupOf(0).size;
}

function task2() {
  const found = new Set();
  let groups = 0;
  while (found.size !== input.length) {
    for (let i = 0; i < input.length; i++) {
      if (!found.has(i)) {
        const group = groupOf(i);
        groups++;
        Array.from(group).forEach((a) => found.add(a));
        break;
      }
    }
  }

  return groups;
}

console.log(task1());
console.log(task2());
