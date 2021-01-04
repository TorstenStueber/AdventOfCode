const { readFileSync } = require("fs");
const { join } = require("path");

const rawInput = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");
const input = rawInput.slice(1);
const binding = Number(/#ip (\d)/.exec(rawInput[0])[1]);

/*
  This is the input program translated into JS:

  let r3 = 123;
  do {
    r3 &= 456;
  } while (r3 !== 72)

  r3 = 0;
  do {
    let r2 = r3 | 65536;
    r3 = 14070682;
    while (true) {
      r3 = (((r3 + (r2 & 255)) & 16777215) * 65899) & 16777215;
      if (r2 < 256) break;
      let r1 = 0;
      while ((r1 + 1) * 256 <= r2) r1++;
      r2 = r1;
    }
  } while (r3 !== r0);
*/

function executeRound(r3) {
  let r2 = r3 | 65536;
  r3 = 14070682;
  while (true) {
    r3 = (((r3 + (r2 & 255)) & 16777215) * 65899) & 16777215;
    if (r2 < 256) break;
    let r1 = 0;
    while ((r1 + 1) * 256 <= r2) r1++;
    r2 = r1;
  }
  return r3;
}

function task1() {
  return executeRound(0);
}

function task2() {
  const seen = new Set();
  let r3 = 0;
  while (true) {
    previousR3 = r3;
    r3 = executeRound(r3);
    if (seen.has(r3)) return previousR3;
    seen.add(r3);
  }
}

console.log(task1());
console.log(task2());
