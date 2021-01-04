const { strict } = require("assert");
const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => {
    const [_, ing, al] = /([\w ]+) \(contains ([\w, ]+)\)/.exec(a);
    return { ing: ing.split(" "), al: al.split(", ") };
  });

const ings = {};
input.forEach((line, i) => {
  line.ing.forEach((ing) => {
    if (ings[ing] === undefined) ings[ing] = new Set();
    ings[ing].add(i);
  });
});

const cantHave = [];
Object.keys(ings).forEach((ing) => {
  const canHave = Array.from(ings[ing]).some((i) =>
    input[i].al.some((al) => {
      const r = input.every((line) => {
        if (line.al.indexOf(al) === -1) return true;
        return line.ing.indexOf(ing) >= 0;
      });
      return r;
    })
  );
  if (!canHave) cantHave.push(ing);
});

function task1() {
  let c = 0;
  input.forEach((line) => {
    line.ing.forEach((ing) => {
      if (cantHave.indexOf(ing) >= 0) c++;
    });
  });

  return c;
}

function task2() {
  const input2 = input.map(({ ing, al }) => ({ ing: ing.filter((i) => cantHave.indexOf(i) === -1), al }));

  const foundAls = {};
  const foundIngs = [];
  const totalAls = new Set();
  input2.forEach((line) => line.al.forEach((al) => totalAls.add(al)));

  while (Object.keys(foundAls).length !== totalAls.size) {
    Array.from(totalAls).some((al) => {
      if (foundAls[al] !== undefined) return false;

      const candidates = new Set();
      input2.forEach((line) => {
        if (line.al.indexOf(al) >= 0) {
          line.ing.forEach((ing) => {
            if (foundIngs.indexOf(ing) === -1) candidates.add(ing);
          });
        }
      });

      const filtered = Array.from(candidates).filter((ing) => {
        return input2.every((line) => {
          if (line.al.indexOf(al) === -1) return true;
          return line.ing.indexOf(ing) >= 0;
        });
      });

      if (filtered.length === 1) {
        foundIngs.push(filtered[0]);
        foundAls[al] = filtered[0];
        return true;
      }
    });
  }

  return Object.entries(foundAls)
    .sort((a, b) => (a[0] < b[0] ? -1 : 1))
    .map((a) => a[1])
    .join(",");
}

console.log(task1());
console.log(task2());
