const { strict } = require("assert");
const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n\n")
  .map((a) => a.split("\n"));

function fill(input, ignoreRecursiveRules) {
  const found = {};
  while (Object.keys(found).length < input[0].length) {
    const foundSome = input[0].some((line) => {
      if (
        found[line.split(":")[0]] !== undefined ||
        (ignoreRecursiveRules && (line.split(":")[0] === "8" || line.split(":")[0] === "11"))
      )
        return false;
      const r = line.split(":")[1].trim().replace(/\| /g, "").split(" ");
      if (r[0].startsWith('"')) {
        found[line.split(":")[0]] = [r[0].replace(/"/g, "")];
        return true;
      } else {
        if (r.every((rr) => found[rr.trim()] !== undefined)) {
          found[line.split(":")[0]] = [];
          line
            .split(":")[1]
            .split("|")
            .forEach((ss) => {
              const sr = ss.trim().split(" ");
              if (sr.length === 1) {
                found[line.split(":")[0]] = [...found[line.split(":")[0]], ...found[sr[0].trim()]];
              } else if (sr.length === 2) {
                found[sr[0].trim()].forEach((f0) => {
                  found[sr[1].trim()].forEach((f1) => {
                    found[line.split(":")[0]].push(f0 + f1);
                  });
                });
              } else if (sr.length === 3) {
                found[sr[0].trim()].forEach((f0) => {
                  found[sr[1].trim()].forEach((f1) => {
                    found[sr[2].trim()].forEach((f2) => {
                      found[line.split(":")[0]].push(f0 + f1 + f2);
                    });
                  });
                });
              }
            });
          return true;
        }
      }
    });
    if (!foundSome) break;
  }

  return found;
}

function task1() {
  const found = new Set(fill(input, false)["0"]);
  return input[1].filter((line) => found.has(line.trim())).length;
}

function match(left, rule, others, found) {
  if (found[rule] !== undefined) {
    return found[rule].some((rr) => {
      if (left.startsWith(rr)) {
        if (others.length === 0) {
          return left.length === rr.length;
        }
        return match(left.slice(rr.length), others[0], others.slice(1), found);
      }
    });
  }

  const ll =
    rule === "8"
      ? "42 | 42 8"
      : rule === "11"
      ? "42 31 | 42 11 31"
      : input[0].find((line) => line.split(":")[0] === rule).split(":")[1];

  return ll.split("|").some((ss) => {
    const sr = ss.trim().split(" ");
    return match(left, sr[0], [...sr.slice(1), ...others], found);
  });
}

function task2() {
  const found = fill(input, true);
  return input[1].filter((line) => match(line, "0", [], found)).length;
}

console.log(task1());
console.log(task2());
