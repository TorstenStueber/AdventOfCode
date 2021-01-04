const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .split("\n")
  .map((b) =>
    b.split(", ").map((x) =>
      x
        .slice(3, -1)
        .split(",")
        .map((a) => Number(a))
    )
  );

function evolve(state) {
  const updated = state.map(([[xP, yP, zP], [xV, yV, zV], [xA, yA, zA]]) => [
    [xP + xV + xA, yP + yV + yA, zP + zV + zA],
    [xV + xA, yV + yA, zV + zA],
    [xA, yA, zA],
  ]);

  const positions = {};
  updated.forEach(([[xP, yP, zP]], i) => {
    const key = `${xP};${yP};${zP}`;
    if (positions[key] === undefined) positions[key] = [];
    positions[key].push(i);
  });

  return updated.filter(([[xP, yP, zP]]) => {
    const key = `${xP};${yP};${zP}`;
    return positions[key].length === 1;
  });
}

function sameSign2(a, b) {
  return (a >= 0 && b >= 0) || (a <= 0 && b <= 0);
}

function sameSign3(a, b, c) {
  return sameSign2(a, b) && sameSign2(a, c) && sameSign2(b, c);
}

function isSettled(state) {
  return state.every(
    ([[xP, yP, zP], [xV, yV, zV], [xA, yA, zA]]) =>
      sameSign3(xA, xV, xP) && sameSign3(yA, yV, yP) && sameSign3(zA, zV, zP)
  );
}

function task1() {
  let minIndexes = [];
  let min = Infinity;
  input.forEach(([_p, _v, [xA, yA, zA]], i) => {
    const a = Math.abs(xA) + Math.abs(yA) + Math.abs(zA);
    if (a < min) {
      min = a;
      minIndexes = [];
    }

    if (a === min) {
      minIndexes.push(i);
    }
  });

  let candidates = input.filter((_, i) => minIndexes.includes(i));
  while (!isSettled(candidates)) {
    candidates = evolve(candidates);
  }

  let minIndex;
  min = Infinity;
  candidates.forEach(([_p, [xV, yV, zV]], i) => {
    const v = Math.abs(xV) + Math.abs(yV) + Math.abs(zV);
    if (v < min) {
      min = v;
      minIndex = minIndexes[i];
    }
  });

  return minIndex;
}

function task2() {
  let state = input;
  for (let i = 0; i < 1000; i++) {
    state = evolve(state);
  }
  return state.length;
}

console.log(task1());
console.log(task2());
