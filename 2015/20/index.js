const { readFileSync, chmod } = require("fs");
const { join } = require("path");

const input = Number(readFileSync(join(__dirname, "input.txt"), "utf-8").trim());

const primes = {};

function isPrime(n) {
  if (n === 1) return false;
  if (primes[n] !== undefined) return primes[n];

  let max = Math.floor(Math.sqrt(n));
  for (let i = 2; i <= max; i++) {
    if (n % i === 0) {
      primes[n] = false;
      return false;
    }
  }

  primes[n] = true;
  return true;
}

function factorize(n) {
  const factors = {};
  let sqr = Math.floor(Math.sqrt(n));
  for (let i = 2; i <= n; i++) {
    if (i > sqr && i !== n) {
      if (isPrime(n)) factors[n] = 1;
      break;
    }

    if (!isPrime(i)) continue;
    while (n % i === 0) {
      if (factors[i] === undefined) factors[i] = 0;
      factors[i]++;
      n /= i;
      sqr = Math.floor(Math.sqrt(n));
    }
  }
  return factors;
}

function task1() {
  let i = 1;
  let max = 0;
  while (true) {
    const facts = factorize(i);
    let p = 1;
    Object.entries(facts).forEach(([i, n]) => (p *= (Number(i) ** (n + 1) - 1) / (Number(i) - 1)));
    max = Math.max(max, p);
    //if (i % 100000 === 0) console.log(i, max);
    if (p * 10 >= input) return i;
    i++;
  }
}

function task2() {
  let i = 1;
  let max = 0;

  while (true) {
    const facts = factorize(i);

    let sum = 0;

    function rec(left, p) {
      if (left.length === 0) {
        if (i / p <= 50) sum += p;
      }
      const q = left[0];
      for (let j = 0; j <= facts[q]; j++) {
        rec(left.slice(1), p * q ** j);
      }
    }

    rec(Object.keys(facts), 1);

    max = Math.max(max, sum);
    //if (i % 100000 === 0) console.log(i, max);
    if (sum * 11 >= input) return i;
    i++;
  }
}

console.log(task1());
console.log(task2());
