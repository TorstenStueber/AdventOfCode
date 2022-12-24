const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n");

function strength(overwriteB) {
  const computed = {};
  if (overwriteB !== undefined) {
    computed["b"] = overwriteB;
  }
  while (computed["a"] === undefined) {
    const x = input.some((line) => {
      const [inp, out] = line.split(" -> ");
      if (computed[out] !== undefined) return false;

      if (/^([a-z]+)$/.exec(inp)) {
        if (computed[inp] === undefined) return false;
        computed[out] = computed[inp];
        return true;
      }

      if (/^(\d+)$/.exec(inp)) {
        computed[out] = Number(inp);
        return true;
      }

      let [_, op1, op, op2] = /^([\da-z]+)? ?(AND|NOT|LSHIFT|RSHIFT|OR) ([\da-z]+)$/.exec(inp);
      if (op1 !== undefined && /^([a-z]+)$/.exec(op1) && computed[op1] === undefined) return false;
      op1 = /^([a-z]+)$/.exec(op1) ? computed[op1] : Number(op1);
      if (/^([a-z]+)$/.exec(op2) && computed[op2] === undefined) return false;
      op2 = /^([a-z]+)$/.exec(op2) ? computed[op2] : Number(op2);

      switch (op) {
        case "AND":
          computed[out] = op1 & op2;
          break;
        case "OR":
          computed[out] = op1 | op2;
          break;
        case "NOT":
          computed[out] = ~op2;
          break;
        case "RSHIFT":
          computed[out] = op1 >> op2;
          break;
        case "LSHIFT":
          computed[out] = (op1 << op2) & 0xffff;
          break;
      }

      return true;
    });
  }

  return computed["a"];
}

function task1() {
  return strength();
}

function task2() {
  return strength(strength());
}

console.log(task1());
console.log(task2());
