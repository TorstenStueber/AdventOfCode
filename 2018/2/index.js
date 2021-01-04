const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  let c2 = 0;
  let c3 = 0;
  input.forEach((b) => {
    const f = {};
    b.split("").forEach((a) => {
      if (f[a] === undefined) f[a] = 0;
      f[a]++;
    });
    if (Object.values(f).some((n) => n === 2)) c2++;
    if (Object.values(f).some((n) => n === 3)) c3++;
  });

  return c2 * c3;
}

function task2() {
  for (let i = 0; i < input.length; i++) {
    for (let j = i + 1; j < input.length; j++) {
      if (input[i].split("").filter((a, k) => input[j][k] !== a).length === 1) {
        return input[i]
          .split("")
          .filter((a, k) => input[j][k] === a)
          .join("");
      }
    }
  }
}

console.log(task1());
console.log(task2());
