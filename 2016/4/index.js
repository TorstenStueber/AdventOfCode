const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((line) => {
    const [a, b] = line.split("[");
    return {
      code: a.split("-").slice(0, -1),
      id: Number(a.split("-").slice(-1)[0]),
      cs: b.slice(0, -1),
    };
  });

function task1() {
  return input.reduce((a, { code, id, cs }) => {
    const f = {};

    code
      .join("")
      .split("")
      .forEach((a) => {
        if (f[a] === undefined) f[a] = 0;
        f[a]++;
      });

    const cs2 = Object.keys(f)
      .sort((a, b) => (f[a] === f[b] ? (a < b ? -1 : 1) : f[b] - f[a]))
      .slice(0, 5);

    if (cs === cs2.join("")) return a + id;
    return a;
  }, 0);
}

const aCode = "a".charCodeAt(0);

function task2() {
  return input.map(
    ({ code, id }) =>
      code
        .map((fragment) =>
          fragment
            .split("")
            .map((a) => {
              return String.fromCharCode(aCode + ((a.charCodeAt(0) - aCode + id) % 26));
            })
            .join("")
        )
        .join(" ") + `: ${id}`
  );
}

console.log(task1());
console.log(JSON.stringify(task2(), null, 2));
