const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

function task1() {
  let result = "";
  for (let i = 0; i < input.length; ) {
    if (input[i] === "(") {
      let [p, l, n] = /\((\d+)x(\d+)\)/.exec(input.slice(i));
      l = Number(l);
      n = Number(n);

      while (n > 0) {
        n--;
        result += input.slice(i + p.length, i + p.length + l);
      }

      i += p.length + l;
    } else {
      result += input[i];
      i++;
    }
  }

  return result.length;
}

function task2() {
  function decompressLength(slice) {
    let resultLength = 0;

    for (let i = 0; i < slice.length; ) {
      if (slice[i] === "(") {
        let [p, l, n] = /\((\d+)x(\d+)\)/.exec(slice.slice(i));
        l = Number(l);
        n = Number(n);

        const subLength = decompressLength(slice.slice(i + p.length, i + p.length + l));
        resultLength += subLength * n;

        i += p.length + l;
      } else {
        resultLength++;
        i++;
      }
    }

    return resultLength;
  }

  return decompressLength(input);
}

console.log(task1());
console.log(task2());
