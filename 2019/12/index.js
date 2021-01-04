const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => /^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$/.exec(a))
  .map(([_, x, y, z]) => ({ x: Number(x), y: Number(y), z: Number(z), xv: 0, yv: 0, zv: 0 }));

function gravity(state) {
  return state.map((moon, index) => {
    return state.reduce(
      ({ xv, yv, zv }, moon2, index2) => {
        if (index === index2) return { xv, yv, zv };
        return {
          xv: xv + (moon2.x > moon.x ? 1 : moon2.x < moon.x ? -1 : 0),
          yv: yv + (moon2.y > moon.y ? 1 : moon2.y < moon.y ? -1 : 0),
          zv: zv + (moon2.z > moon.z ? 1 : moon2.z < moon.z ? -1 : 0),
        };
      },
      { xv: moon.xv, yv: moon.yv, zv: moon.zv }
    );
  });
}

function step(state) {
  const speed = gravity(state);
  return state.map((moon, index) => ({
    x: moon.x + speed[index].xv,
    y: moon.y + speed[index].yv,
    z: moon.z + speed[index].zv,
    ...speed[index],
  }));
}

function energy(state) {
  return state.reduce(
    (acc, moon) =>
      acc +
      (Math.abs(moon.x) + Math.abs(moon.y) + Math.abs(moon.z)) *
        (Math.abs(moon.xv) + Math.abs(moon.yv) + Math.abs(moon.zv)),
    0
  );
}

function task1() {
  let state = input;
  for (let i = 0; i < 1000; i++) {
    state = step(state);
  }
  return energy(state);
}

function findRepeats(initialState) {
  const states = {};
  let state = initialState;
  for (let i = 0; true; i++) {
    const key = state.map(({ x, y, z, xv, yv, zv }) => `${x},${y},${z},${xv},${yv},${zv}`).join(";");
    if (states[key] !== undefined) return i;
    states[key] = i;
    state = step(state);
  }
}

function euclid(a, b) {
  while (b !== 0) {
    x = a % b;
    a = b;
    b = x;
  }
  return a;
}

function kgV(a, b) {
  return (a * b) / euclid(a, b);
}

function task2() {
  const xR = findRepeats(input.map(({ x }) => ({ x, y: 0, z: 0, xv: 0, yv: 0, zv: 0 })));
  const yR = findRepeats(input.map(({ y }) => ({ x: 0, y, z: 0, xv: 0, yv: 0, zv: 0 })));
  const zR = findRepeats(input.map(({ z }) => ({ x: 0, y: 0, z, xv: 0, yv: 0, zv: 0 })));

  return kgV(kgV(xR, yR), zR);
}

console.log(task1());
console.log(task2());
