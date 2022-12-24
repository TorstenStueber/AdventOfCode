const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => {
    const n = a.split(":")[0].split(" ")[1];
    const list = a
      .split(":")
      .slice(1)
      .join(":")
      .trim()
      .split(", ")
      .reduce((a, b) => ({ ...a, [b.split(": ")[0]]: Number(b.split(": ")[1]) }), {});
    return { n, list };
  });

const list = `children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1`
  .split("\n")
  .reduce((a, b) => ({ ...a, [b.split(": ")[0]]: Number(b.split(": ")[1]) }), {});

function task1() {
  return (
    1 +
    input.findIndex((sue) => {
      return Object.entries(sue.list).every(([key, no]) => list[key] === no);
    })
  );
}

function task2() {
  return (
    1 +
    input.findIndex((sue) => {
      return Object.entries(sue.list).every(([key, no]) =>
        key === "cats" || key === "trees"
          ? list[key] < no
          : key === "pomeranians" || key === "goldfish"
          ? list[key] > no
          : list[key] === no
      );
    })
  );
}

console.log(task1());
console.log(task2());
