const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function step(pw, line) {
  const swapPos = /swap position (\d+) with position (\d+)/.exec(line);
  const swapLet = /swap letter (\w) with letter (\w)/.exec(line);
  const rotate = /rotate (left|right) (\d+) steps?/.exec(line);
  const rotateLet = /rotate based on position of letter (\w)/.exec(line);
  const reverse = /reverse positions (\d+) through (\d+)/.exec(line);
  const move = /move position (\d+) to position (\d+)/.exec(line);

  if (swapPos) {
    const newPw = [...pw];
    newPw[Number(swapPos[1])] = pw[Number(swapPos[2])];
    newPw[Number(swapPos[2])] = pw[Number(swapPos[1])];
    return newPw;
  }
  if (swapLet) {
    return pw.map((a) => (a === swapLet[1] ? swapLet[2] : a === swapLet[2] ? swapLet[1] : a));
  }
  if (rotate) {
    const steps = rotate[1] === "right" ? pw.length - Number(rotate[2]) : Number(rotate[2]);
    return [...pw.slice(steps), ...pw.slice(0, steps)];
  }
  if (rotateLet) {
    const index = pw.findIndex((a) => a === rotateLet[1]);
    const rightSteps = (index + 1 + (index >= 4 ? 1 : 0)) % pw.length;
    const steps = pw.length - rightSteps;
    return [...pw.slice(steps), ...pw.slice(0, steps)];
  }
  if (reverse) {
    const x = Number(reverse[1]);
    const y = Number(reverse[2]);
    return [...pw.slice(0, x), ...[...pw.slice(x, y + 1)].reverse(), ...pw.slice(y + 1)];
  }
  if (move) {
    const x = Number(move[1]);
    const y = Number(move[2]);
    const newPw = [...pw];
    newPw.splice(x, 1);
    return [...newPw.slice(0, y), pw[x], ...newPw.slice(y)];
  }
}

function unStep(pw, line) {
  const swapPos = /swap position (\d+) with position (\d+)/.exec(line);
  const swapLet = /swap letter (\w) with letter (\w)/.exec(line);
  const rotate = /rotate (left|right) (\d+) steps?/.exec(line);
  const rotateLet = /rotate based on position of letter (\w)/.exec(line);
  const reverse = /reverse positions (\d+) through (\d+)/.exec(line);
  const move = /move position (\d+) to position (\d+)/.exec(line);

  if (swapPos) {
    const newPw = [...pw];
    newPw[Number(swapPos[1])] = pw[Number(swapPos[2])];
    newPw[Number(swapPos[2])] = pw[Number(swapPos[1])];
    return newPw;
  }
  if (swapLet) {
    return pw.map((a) => (a === swapLet[1] ? swapLet[2] : a === swapLet[2] ? swapLet[1] : a));
  }
  if (rotate) {
    const steps = rotate[1] === "left" ? pw.length - Number(rotate[2]) : Number(rotate[2]);
    return [...pw.slice(steps), ...pw.slice(0, steps)];
  }
  if (rotateLet) {
    const index = pw.findIndex((a) => a === rotateLet[1]);
    const steps = index % 2 === 0 ? ((index / 2 - 1 + pw.length / 2) % (pw.length / 2)) + 6 : (index + 1) / 2;
    return [...pw.slice(steps % pw.length), ...pw.slice(0, steps % pw.length)];
  }
  if (reverse) {
    const x = Number(reverse[1]);
    const y = Number(reverse[2]);
    return [...pw.slice(0, x), ...[...pw.slice(x, y + 1)].reverse(), ...pw.slice(y + 1)];
  }
  if (move) {
    const y = Number(move[1]);
    const x = Number(move[2]);
    const newPw = [...pw];
    newPw.splice(x, 1);
    return [...newPw.slice(0, y), pw[x], ...newPw.slice(y)];
  }
}

function task1() {
  let pw = "abcdefgh".split("");
  input.forEach((line) => {
    pw = step(pw, line);
  });
  return pw.join("");
}

function task2() {
  let pw = "fbgdceah".split("");
  [...input].reverse().forEach((line) => {
    pw = unStep(pw, line);
  });
  return pw.join("");
}

console.log(task1());
console.log(task2());
