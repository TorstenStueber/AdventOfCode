const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => /Step (\w) must be finished before step (\w) can begin./.exec(a).slice(1));

function getCounts() {
  const counts = [];
  input.forEach(([s1, s2]) => {
    if (counts[s2] === undefined) counts[s2] = 0;
    counts[s2]++;

    if (counts[s1] === undefined) counts[s1] = 0;
  });

  return counts;
}

function task1() {
  const counts = getCounts();
  let sel = "";

  while (Object.keys(counts).length) {
    const selected = Object.keys(counts)
      .filter((key) => counts[key] === 0)
      .sort()[0];

    sel += selected;
    input.forEach(([s1, s2]) => {
      if (s1 === selected) counts[s2]--;
    });

    delete counts[selected];
  }

  return sel;
}

function task2() {
  const counts = getCounts();

  const noOfWorkers = 5;

  const workers = [];
  for (let i = 0; i < noOfWorkers; i++) workers.push(undefined);
  let seconds = 0;

  while (true) {
    while (workers.indexOf(undefined) !== -1) {
      const selected = Object.keys(counts)
        .filter((key) => counts[key] === 0)
        .sort()[0];

      if (selected !== undefined) {
        const worker = workers.indexOf(undefined);
        workers[worker] = { left: 60 + selected.charCodeAt(0) - "A".charCodeAt(0) + 1, stage: selected };
        delete counts[selected];
      } else {
        if (Object.keys(counts).length === 0 && workers.filter((a) => a !== undefined).length === 0) {
          return seconds;
        }
        break;
      }
    }

    seconds++;
    for (let i = 0; i < noOfWorkers; i++) {
      if (workers[i] !== undefined) {
        workers[i].left--;
        if (workers[i].left === 0) {
          input.forEach(([s1, s2]) => {
            if (s1 === workers[i].stage) counts[s2]--;
          });
          workers[i] = undefined;
        }
      }
    }
  }
}

console.log(task1());
console.log(task2());
