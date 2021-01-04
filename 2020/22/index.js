const { strict } = require("assert");
const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n\n")
  .map((a) =>
    a
      .split("\n")
      .slice(1)
      .map((b) => Number(b))
  );

function task1() {
  let list1 = [...input[0]];
  let list2 = [...input[1]];
  while (list1.length && list2.length) {
    const a = list1.shift();
    const b = list2.shift();
    if (a > b) {
      list1.push(a);
      list1.push(b);
    } else {
      list2.push(b);
      list2.push(a);
    }
  }
  const winner = list1.length ? list1 : list2;
  return winner.reduce((a, b, i) => a + b * (winner.length - i), 0);
}

function task2() {
  function rec(list1, list2) {
    const seen1 = new Set();
    const seen2 = new Set();

    while (list1.length && list2.length) {
      if (seen1.has(list1.join(",")) && seen2.has(list2.join(","))) {
        return { list: list1, n: 1 };
      }

      seen1.add(list1.join(","));
      seen2.add(list2.join(","));

      const a = list1.shift();
      const b = list2.shift();

      let winner;
      if (a <= list1.length && b <= list2.length) {
        const r = rec([...list1.slice(0, a)], [...list2.slice(0, b)]);
        winner = r.n;
      } else {
        winner = a > b ? 1 : 2;
      }

      if (winner === 1) {
        list1.push(a);
        list1.push(b);
      } else {
        list2.push(b);
        list2.push(a);
      }
    }
    return list1.length ? { list: list1, n: 1 } : { list: list2, n: 2 };
  }

  const winner = rec([...input[0]], [...input[1]]).list;
  return winner.reduce((a, b, i) => a + b * (winner.length - i), 0);
}

console.log(task1());
console.log(task2());
