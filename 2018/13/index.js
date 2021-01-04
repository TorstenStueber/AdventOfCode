const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .split("\n")
  .map((a) => a.split(""));

const dirs = [
  [0, -1],
  [1, 0],
  [0, 1],
  [-1, 0],
];

function init() {
  const carts = [];
  let id = 0;
  input.forEach((line, y) => {
    line.forEach((a, x) => {
      if (["^", ">", "v", "<"].includes(a)) {
        carts.push({ id: id++, dir: ["^", ">", "v", "<"].indexOf(a), x, y, phase: 0 });
      }
    });
  });
  return carts;
}

function tick(carts, crashed) {
  let collisionX;
  let collisionY;
  let stopped = false;
  [...carts]
    .filter(({ id }) => !crashed.includes(id))
    .sort(({ x: x1, y: y1 }, { x: x2, y: y2 }) => {
      if (y1 < y2) {
        return -1;
      }
      if (y2 < y1) {
        return 1;
      }
      return x1 - x2;
    })
    .forEach((cart, _, ordered) => {
      if (stopped) return;
      if (input[cart.y][cart.x] === "+") {
        switch (cart.phase % 3) {
          case 0:
            cart.dir = (cart.dir + 3) % 4;
            break;
          case 2:
            cart.dir = (cart.dir + 1) % 4;
            break;
        }
        cart.phase++;
      } else if (input[cart.y][cart.x] === "/") {
        cart.dir = [1, 0, 3, 2][cart.dir];
      } else if (input[cart.y][cart.x] === "\\") {
        cart.dir = [3, 2, 1, 0][cart.dir];
      }
      cart.x += dirs[cart.dir][0];
      cart.y += dirs[cart.dir][1];
      ordered.forEach((cart2) => {
        if (cart.id !== cart2.id && cart.x === cart2.x && cart.y === cart2.y) {
          if (collisionX === undefined) {
            collisionX = cart.x;
            collisionY = cart.y;
          }
          crashed.push(cart.id);
          crashed.push(cart2.id);

          if (carts.length - 1 === crashed.length) {
            stopped = true;
          }
        }
      });
    });

  return { collisionX, collisionY };
}

function task1() {
  const state = init();
  const crashed = [];
  while (true) {
    const { collisionX, collisionY } = tick(state, crashed);
    if (collisionX !== undefined) return `${collisionX},${collisionY}`;
  }
}

function task2() {
  const state = init();
  const crashed = [];
  while (state.length - 1 !== crashed.length) {
    tick(state, crashed);
  }

  const cart = state.find((_, i) => !crashed.includes(i));
  return `${cart.x},${cart.y}`;
}

console.log(task1());
console.log(task2());
