const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => {
    const s = a.split(" => ");
    return {
      i: s[0].split(", ").map(numberChemical),
      o: numberChemical(s[1]),
    };
  });

function numberChemical(ingredient) {
  const m = /^(\d+) (.*)$/.exec(ingredient);
  return { n: m[1], c: m[2] };
}

function chemicals() {
  return [...input.map((r) => r.o.c), "ORE"];
}

function fuelCount(fuelCount) {
  const cc = chemicals();
  const inputCount = {};
  cc.forEach((c) => {
    inputCount[c] = input.filter((r) => r.i.some((i) => i.c === c)).length;
  });

  inputCount["FUEL"] = 0;
  const required = { FUEL: fuelCount };

  while (true) {
    const selected = Object.keys(inputCount).find((key) => inputCount[key] === 0);
    delete inputCount[selected];

    if (selected === "ORE") return required[selected];

    const rule = input.find((r) => r.o.c === selected);

    const n = Math.ceil((required[selected] ?? 0) / rule.o.n);

    rule.i.forEach((i) => {
      inputCount[i.c]--;
      if (required[i.c] === undefined) required[i.c] = 0;
      required[i.c] += n * i.n;
    });
  }
}

function task1() {
  return fuelCount(1);
}

function task2() {
  const limit = 1000000000000;

  let fuelTop = 1;
  while (fuelCount(fuelTop) < limit) fuelTop *= 2;
  let fuelBot = 1;

  while (fuelTop !== fuelBot) {
    const middle = Math.floor((fuelTop - fuelBot) / 2) + fuelBot;
    if (fuelCount(middle) <= limit) {
      fuelBot = middle;
    } else {
      fuelTop = middle;
    }
    if (fuelBot + 1 === fuelTop) {
      return fuelCount(fuelTop) <= limit ? fuelTop : fuelBot;
    }
  }

  return fuelBot;
}

console.log(task1());
console.log(task2());
