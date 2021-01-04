const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => {
    const [_, n1, type, diff, n2] = /(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\./.exec(a);
    return { n1, n2, type, diff: Number(diff) };
  });

const people = Array.from(
  input.reduce((a, b) => {
    a.add(b.n1);
    a.add(b.n2);
    return a;
  }, new Set())
);

function task1() {
  let max = 0;
  function recurse(used) {
    if (used.length === people.length) {
      let s = 0;
      used.forEach((b, i) => {
        const b2 = i === used.length - 1 ? used[0] : used[i + 1];
        const f1 = input.find(({ n1, n2 }) => n1 === b && n2 === b2);
        s += f1.type === "lose" ? -f1.diff : f1.diff;
        const f2 = input.find(({ n1, n2 }) => n1 === b2 && n2 === b);
        s += f2.type === "lose" ? -f2.diff : f2.diff;
      });

      max = Math.max(max, s);
      return;
    }

    people.forEach((t) => {
      if (used.includes(t)) return;
      recurse([...used, t]);
    });
  }

  recurse([people[0]]);
  return max;
}

function task2() {
  let max = 0;
  function recurse(used) {
    if (used.length === people.length + 1) {
      let s = 0;
      used.forEach((b, i) => {
        if (i === 0 || i === used.length - 1) return;
        const b2 = i === used.length - 1 ? used[0] : used[i + 1];
        const f1 = input.find(({ n1, n2 }) => n1 === b && n2 === b2);
        s += f1.type === "lose" ? -f1.diff : f1.diff;
        const f2 = input.find(({ n1, n2 }) => n1 === b2 && n2 === b);
        s += f2.type === "lose" ? -f2.diff : f2.diff;
      });

      max = Math.max(max, s);
      return;
    }

    people.forEach((t) => {
      if (used.includes(t)) return;
      recurse([...used, t]);
    });
  }

  recurse(["me"]);
  return max;
}

console.log(task1());
console.log(task2());
