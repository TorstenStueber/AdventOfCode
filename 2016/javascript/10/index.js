const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function run() {
  const bots = {};
  const outputs = {};
  input.forEach((line) => {
    const inp = /value (\d+) goes to bot (\d+)/.exec(line);
    if (inp) {
      const v = Number(inp[1]);
      const i = Number(inp[2]);
      if (bots[i] === undefined) bots[i] = [];
      bots[i].push(v);
    }
  });

  let foundBot;
  let change;
  do {
    change = false;
    input.forEach((line) => {
      const inst = /bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)/.exec(line);
      if (inst) {
        const i = Number(inst[1]);
        const o1 = inst[2];
        const l = Number(inst[3]);
        const o2 = inst[4];
        const h = Number(inst[5]);

        if (
          bots[i] &&
          bots[i].length === 2 &&
          (o1 === "output" || bots[l] === undefined || bots[l].length < 2) &&
          (o2 === "output" || bots[h] === undefined || bots[h].length < 2)
        ) {
          if (o1 === "bot" && bots[l] === undefined) bots[l] = [];
          if (o2 === "bot" && bots[h] === undefined) bots[h] = [];

          const [low, high] = bots[i].sort((a, b) => a - b);

          if (low === 17 && high === 61) {
            foundBot = i;
          }

          if (o1 === "bot") {
            bots[l].push(low);
          } else {
            outputs[l] = low;
          }

          if (o2 === "bot") {
            bots[h].push(high);
          } else {
            outputs[h] = high;
          }

          bots[i] = [];
          change = true;
        }
      }
    });
  } while (change);

  return { foundBot, outputs };
}

function task1() {
  const { foundBot } = run();

  return foundBot;
}

function task2() {
  const { outputs } = run();

  return outputs[0] * outputs[1] * outputs[2];
}

console.log(task1());
console.log(task2());
