const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => a.split(" "));

function task1() {
  return input.reduce((a, phrase) => a + (phrase.some((word, i) => phrase.slice(i + 1).includes(word)) ? 0 : 1), 0);
}

function task2() {
  const orderedInput = input.map((line) =>
    line.map((word) =>
      word
        .split("")
        .sort((a, b) => (a < b ? -1 : a === b ? 0 : 1))
        .join("")
    )
  );

  return orderedInput.reduce(
    (a, phrase) => a + (phrase.some((word, i) => phrase.slice(i + 1).includes(word)) ? 0 : 1),
    0
  );
}

console.log(task1());
console.log(task2());
