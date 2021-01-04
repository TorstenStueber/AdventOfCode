const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split(",");

const length = 16;

function task1() {
  let state = [];
  for (let i = 0; i < length; i++) state.push(String.fromCharCode(i + "a".charCodeAt(0)));
  input.forEach((c) => {
    const spin = /s(\d+)/.exec(c);
    const ex = /x(\d+)\/(\d+)/.exec(c);
    const part = /p(\w)\/(\w)/.exec(c);
    if (spin) {
      let d = spin[1];
      while (d--) {
        state = [state[length - 1], ...state.slice(0, length - 1)];
      }
    } else if (ex) {
      const [_, p1, p2] = ex;
      const x = state[p1];
      state[p1] = state[p2];
      state[p2] = x;
    } else if (part) {
      const [_, p1, p2] = part;
      state = state.map((a) => (a === p1 ? p2 : a === p2 ? p1 : a));
    }
  });

  return state.join("");
}

function task2() {
  let letterAt = [];
  let whereLetter = {};

  const repeats = 1000000000;

  for (let i = 0; i < length; i++) {
    const letter = String.fromCharCode(i + "a".charCodeAt(0));
    letterAt[i] = letter;
    whereLetter[letter] = i;
  }

  let start = 0;

  let seen = {};
  for (let i = 0; i < repeats; i++) {
    const state = [...letterAt.slice(start), ...letterAt.slice(0, start)].join("");
    if (seen[state] !== undefined) {
      return Object.keys(seen).find((key) => seen[key] === repeats % i);
    }
    seen[state] = i;

    input.forEach((c) => {
      const spin = /s(\d+)/.exec(c);
      const ex = /x(\d+)\/(\d+)/.exec(c);
      const part = /p(\w)\/(\w)/.exec(c);
      if (spin) {
        start = (((start - Number(spin[1])) % length) + length) % length;
      } else if (ex) {
        let [_, p1, p2] = ex;
        p1 = (Number(p1) + start) % length;
        p2 = (Number(p2) + start) % length;
        l1 = letterAt[p1];
        l2 = letterAt[p2];
        letterAt[p1] = l2;
        letterAt[p2] = l1;
        whereLetter[l1] = p2;
        whereLetter[l2] = p1;
      } else if (part) {
        const [_, l1, l2] = part;
        p1 = whereLetter[l1];
        p2 = whereLetter[l2];
        letterAt[p1] = l2;
        letterAt[p2] = l1;
        whereLetter[l1] = p2;
        whereLetter[l2] = p1;
      }
    });
  }

  return [...letterAt.slice(start), ...letterAt.slice(0, start)].join("");
}

console.log(task1());
console.log(task2());
