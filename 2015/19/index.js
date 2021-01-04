const { readFileSync, chmod } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n\n");

const chem = input[1];
const rules = input[0].split("\n").map((a) => a.split(" => "));

function task1() {
  const results = new Set();
  rules.forEach((rule) => {
    let i = 0;
    while (chem.indexOf(rule[0], i) >= 0) {
      const pos = chem.indexOf(rule[0], i);
      results.add(`${chem.slice(0, pos)}${rule[1]}${chem.slice(pos + rule[0].length)}`);
      i = pos + 1;
    }
  });

  return results.size;
}

function getChem(s) {
  return s.length > 1 && s[1].toLowerCase() === s[1] ? s.slice(0, 2) : s[0];
}

function task2() {
  const chems = [];
  for (let i = 0; i < chem.length; i += getChem(chem.slice(i)).length) chems.push(getChem(chem.slice(i)));
  const Rns = chems.filter((a) => a === "Rn").length;
  const Ys = chems.filter((a) => a === "Y").length;

  return chems.length - 1 - Rns * 2 - Ys * 2;
}

console.log(task1());
console.log(task2());
