const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n\n");

const rules = input[0].split("\n").map((line) => {
  return line
    .split(":")[1]
    .split("or")
    .map((r) => r.split("-").map((a) => Number(a)));
});

function task1() {
  let count = 0;
  input[2]
    .split("\n")
    .slice(1)
    .forEach((line) => {
      line.split(",").forEach((a) => {
        const b = Number(a);
        if (
          !rules.some((r) => {
            return (r[0][0] <= b && r[0][1] >= b) || (r[1][0] <= b && r[1][1] >= b);
          })
        )
          count += b;
      });
    });

  return count;
}

function task2() {
  let valid = [];
  input[2]
    .split("\n")
    .slice(1)
    .forEach((line) => {
      if (
        !line.split(",").some((a) => {
          const b = Number(a);
          if (
            !rules.some((r) => {
              return (r[0][0] <= b && r[0][1] >= b) || (r[1][0] <= b && r[1][1] >= b);
            })
          )
            return true;
          return false;
        })
      )
        valid.push(line);
    });

  const positions = valid[0].split(",").length;
  const cands = [];
  for (let i = 0; i < positions; i++) {
    let cand = [];
    rules.forEach((r, j) => cand.push(j));
    valid.forEach((line) => {
      const b = Number(line.split(",")[i]);
      cand = cand.filter((ruleI) => {
        const r = rules[ruleI];
        return (r[0][0] <= b && r[0][1] >= b) || (r[1][0] <= b && r[1][1] >= b);
      });
    });
    cands[i] = cand;
  }

  let foundCands = [];
  for (k = 0; k < cands.length; k++) {
    cands.some((cand, candIndex) => {
      if (cand.length === 1 && foundCands.indexOf(candIndex) === -1) {
        foundCands.push(candIndex);
        cands.forEach((candd) => {
          const j = candd.indexOf(cand[0]);
          if (j !== -1 && candd.length !== 1) {
            candd.splice(j, 1);
          }
        });
      }
    });
  }

  const my = input[1].split("\n")[1].split(",");
  let prod = 1;
  cands.forEach((r, candIndex) => {
    if (r[0] < 6) {
      prod *= Number(my[candIndex]);
    }
  });
  return prod;
}

console.log(task1());
console.log(task2());
