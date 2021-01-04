const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .sort()
  .map((line) => {
    const [all, year, month, day, hour, min] = /\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\]/.exec(line);
    return {
      year: Number(year),
      month: Number(month),
      day: Number(day),
      hour: Number(hour),
      min: Number(min),
      inst: line.slice(all.length + 1),
    };
  });

function makeGuards() {
  const guards = {};
  let currentGuard;
  let sleepSince;

  input.forEach(({ year, month, day, hour, min, inst }) => {
    if (inst === "wakes up") {
      if (hour !== 0) throw Error("Unexpected hour");
      if (sleepSince === undefined) throw Error("Unexpected wake up");
      if (guards[currentGuard] === undefined) {
        guards[currentGuard] = { total: 0, minuteCount: [] };
      }
      guards[currentGuard].total += min - sleepSince;
      for (i = sleepSince; i < min; i++) {
        if (guards[currentGuard].minuteCount[i] === undefined) guards[currentGuard].minuteCount[i] = 0;
        guards[currentGuard].minuteCount[i]++;
      }
      sleepSince = undefined;
    } else if (inst === "falls asleep") {
      if (hour !== 0) throw Error("Unexpected hour");
      sleepSince = min;
    } else {
      const guard = /Guard #(\d+) begins shift/.exec(inst);
      currentGuard = Number(guard[1]);
    }
  });

  return guards;
}

function task1() {
  const guards = makeGuards();

  let max = 0;
  let maxId;
  Object.keys(guards).forEach((guard) => {
    const { total } = guards[guard];
    if (total > max) {
      max = total;
      maxId = guard;
    }
  });

  let maxMin = 0;
  let maxMinIndex;
  Object.keys(guards[maxId].minuteCount).forEach((min) => {
    if (guards[maxId].minuteCount[min] > maxMin) {
      maxMin = guards[maxId].minuteCount[min];
      maxMinIndex = min;
    }
  });

  return maxId * maxMinIndex;
}

function task2() {
  const guards = makeGuards();
  let max = 0;
  let maxId;
  let maxMin;

  Object.keys(guards).forEach((guard) => {
    Object.keys(guards[guard].minuteCount).forEach((min) => {
      if (guards[guard].minuteCount[min] > max) {
        max = guards[guard].minuteCount[min];
        maxId = guard;
        maxMin = min;
      }
    });
  });

  return maxId * maxMin;
}

console.log(task1());
console.log(task2());
