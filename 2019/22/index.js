const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function task1() {
  let deck = [];
  for (let i = 0; i < 10007; i++) deck.push(i);

  input.forEach((line) => {
    const deal = /deal into new stack/.exec(line);
    const inc = /deal with increment (\d+)/.exec(line);
    const cut = /cut (-?\d+)/.exec(line);
    if (deal) {
      deck.reverse();
    }
    if (inc) {
      const n = Number(inc[1]);
      const newDeck = [];
      deck.forEach((a, i) => {
        newDeck[(i * n) % deck.length] = a;
      });
      deck = newDeck;
    }
    if (cut) {
      const n = Number(cut[1]);
      deck = [...deck.slice(n), ...deck.slice(0, n)];
    }
  });

  return deck.findIndex((a) => a === 2019);
}

function extendedEuclid(a, b) {
  if (b > a) {
    const { n, m } = extendedEuclid(b, a);
    return { n: m, m: n };
  }
  if (b === 0n) return { n: 1n, m: 0n };
  const { n, m } = extendedEuclid(b, a % b);
  // m * (a % b) + n * b = m * (a - (a - a % b) / b * b) + n * b
  // = m * a + (n - m * (a - a % b) / b) * b
  return { n: m, m: n - m * ((a - (a % b)) / b) };
}

function invert(a, mod) {
  const { m } = extendedEuclid(mod, a);
  return (m < 0n ? m + mod : m) % mod;
}

function pow(a, b, mod) {
  // a^(2b + 1) = (a^b)^2 * a
  // a^(2b) = (a^b)^2
  if (b === 0n) return 1n;
  const subPow = pow(a, b >> 1n, mod);
  if ((b & 1n) === 1n) return (subPow * subPow * a) % mod;
  return (subPow * subPow) % mod;
}

function iterateLinearFunction(a, b, n, mod) {
  // for n = 3:
  // p = a * (a * (a * x + b) + b) + b
  // = a^3 * x + (a^2 + a^1 + a^0) * b
  // = a^3 * x + (a^3 - 1) / (a - 1) * b
  // in general: a'  = a^n, b' = (a^n - 1) / (a - 1) * b

  const c = invert(a - 1n, mod);
  const aPowN = pow(a, n, mod);
  return { a: aPowN, b: ((aPowN - 1n) * c * b) % mod };
}

function createReverseFunction(size) {
  let a = 1n;
  let b = 0n;
  const revInput = [...input];
  revInput.reverse();

  revInput.forEach((line) => {
    const deal = /deal into new stack/.exec(line);
    const inc = /deal with increment (\d+)/.exec(line);
    const cut = /cut (-?\d+)/.exec(line);
    if (deal) {
      a = -a % size;
      b = (-b - 1n) % size;
    }
    if (inc) {
      const n = BigInt(inc[1]);
      const m = invert(n, size);
      a = (a * m) % size;
      b = (b * m) % size;
    }
    if (cut) {
      const n = Number(cut[1]);
      b = (b + BigInt(n)) % size;
    }
  });

  return { a, b };
}

function task2() {
  const size = 119315717514047n;
  const iterations = 101741582076661n;

  const { a: a1, b: b1 } = createReverseFunction(size);
  const { a, b } = iterateLinearFunction(a1, b1, iterations, size);
  let pos = 2020n;
  let original = (((a * pos + b) % size) + size) % size;

  return Number(original);
}

console.log(task1());
console.log(task2());
