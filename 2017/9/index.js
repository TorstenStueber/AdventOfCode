const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("");

function removeGarbage(line) {
  let result = "";
  let removed = 0;
  let cancel = false;
  let inGarbage = false;
  line.forEach((a) => {
    if (inGarbage) {
      if (cancel) {
        cancel = false;
        return;
      }
      if (a === "!") {
        cancel = true;
      } else if (a === ">") {
        inGarbage = false;
      } else {
        removed++;
      }
      return;
    }
    if (a === "<") {
      inGarbage = true;
    } else {
      result += a;
    }
  });

  return { cleaned: result, removed };
}

function task1() {
  let c = 0;
  const { cleaned } = removeGarbage(input);
  function parseSub(start, level) {
    c += level;
    while (true) {
      if (cleaned[start] === "{") {
        start = parseSub(start + 1, level + 1);
      } else if (cleaned[start] === ",") {
        start++;
      } else if (cleaned[start] === "}") return start + 1;
    }
  }
  parseSub(1, 1);
  return c;
}

function task2() {
  const { removed } = removeGarbage(input);
  return removed;
}

console.log(task1());
console.log(task2());
