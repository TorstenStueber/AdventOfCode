const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n").map(parseRule);

function parseBag(bag) {
  const x = /^(\d+) (\w+ \w+) bags?\.?$/.exec(bag);
  return { c: x[1], n: x[2] };
}

function parseRule(rule) {
  const s = rule.split(" bags contain ");
  return { n: s[0], r: s[1] === "no other bags." ? [] : s[1].split(", ").map(parseBag) };
}

function addBag(bags, bag) {
  if (bags.indexOf(bag) >= 0) return;
  bags.push(bag);
  input.forEach(({ n, r }) => {
    if (r.some(({ n: inner }) => inner === bag)) addBag(bags, n);
  });
}

function task1() {
  const bags = [];
  addBag(bags, "shiny gold");
  return bags.length - 1;
}

function bags(name) {
  const rule = input.find(({ n }) => n === name);
  return 1 + rule.r.reduce((acc, x) => acc + x.c * bags(x.n), 0);
}

function task2() {
  return bags("shiny gold") - 1;
}

console.log(task1());
console.log(task2());
