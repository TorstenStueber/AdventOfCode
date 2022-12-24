const { readFileSync, chmod } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => Number(a));

const sum = input.reduce((a, b) => a + b, 0);
const partSum3 = sum / 3;
const partSum4 = sum / 4;

function task1() {
  let min = Infinity;
  function recurseFront(selected, sum, i, packsLeft) {
    if (sum > partSum3) return;
    if (packsLeft === 0) {
      if (sum !== partSum3) return;

      function findHalf(selected, leftSelected, sum, i) {
        if (sum > partSum3) return false;
        if (sum === partSum3) return true;
        if (i >= input.length) return false;

        if (findHalf(selected, leftSelected, sum, i + 1)) return true;
        if (!selected.includes(input[i])) {
          return findHalf(selected, [...leftSelected, input[i]], sum + input[i], i + 1);
        }
        return false;
      }

      if (!findHalf(selected, [], 0, 0)) return;

      min = Math.min(
        min,
        selected.reduce((a, b) => a * b, 1)
      );
    } else {
      if (i >= input.length) return;
      recurseFront(selected, sum, i + 1, packsLeft);
      recurseFront([...selected, input[i]], sum + input[i], i + 1, packsLeft - 1);
    }
  }

  let j = 1;
  while (min === Infinity) {
    recurseFront([], 0, 0, j++);
  }

  return min;
}

function task2() {
  let min = Infinity;
  function recurseFront(selected, sum, i, packsLeft) {
    if (sum > partSum4) return;
    if (packsLeft === 0) {
      if (sum !== partSum4) return;

      function findTriple2(selected, leftSelected, sum, i) {
        if (sum > partSum4) return false;
        if (sum === partSum4) return true;
        if (i >= input.length) return false;

        if (findTriple2(selected, leftSelected, sum, i + 1)) return true;
        if (!selected.includes(input[i])) {
          return findTriple2(selected, [...leftSelected, input[i]], sum + input[i], i + 1);
        }
        return false;
      }

      function findTriple1(selected, leftSelected, sum, i) {
        if (sum > partSum4) return false;
        if (sum === partSum4) return findTriple2([...selected, leftSelected], [], 0, 0);
        if (i >= input.length) return false;

        if (findTriple1(selected, leftSelected, sum, i + 1)) return true;
        if (!selected.includes(input[i])) {
          return findTriple1(selected, [...leftSelected, input[i]], sum + input[i], i + 1);
        }
        return false;
      }

      if (!findTriple1(selected, [], 0, 0)) return;

      min = Math.min(
        min,
        selected.reduce((a, b) => a * b, 1)
      );
    } else {
      if (i >= input.length) return;
      recurseFront(selected, sum, i + 1, packsLeft);
      recurseFront([...selected, input[i]], sum + input[i], i + 1, packsLeft - 1);
    }
  }

  let j = 1;
  while (min === Infinity) {
    recurseFront([], 0, 0, j++);
  }

  return min;
}

console.log(task1());
console.log(task2());
