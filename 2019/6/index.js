const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => a.split(")"));

function task1() {
  let count = 0;
  let indirects = { COM: 0 };
  let leftOver = [...input];
  while (leftOver.length > 0) {
    const newLeftOver = [];
    leftOver.forEach((line) => {
      if (indirects[line[0]] !== undefined) {
        indirects[line[1]] = indirects[line[0]] + 1;
        count += indirects[line[0]] + 1;
      } else {
        newLeftOver.push(line);
      }
    });
    leftOver = newLeftOver;
  }
  return count;
}

function findCenters(start) {
  const centers = { [start]: 0 };
  while (centers["COM"] === undefined) {
    input.forEach((line) => {
      if (centers[line[1]] !== undefined) {
        centers[line[0]] = centers[line[1]] + 1;
      }
    });
  }
  return centers;
}

function task2() {
  const yous = findCenters("YOU");
  const sans = findCenters("SAN");

  let least = Infinity;
  Object.keys(yous).forEach((you) => {
    if (sans[you] !== undefined) {
      const length = yous[you] - 1 + sans[you] - 1;
      if (length < least) least = length;
    }
  });

  return least;
}

console.log(task1());
console.log(task2());
