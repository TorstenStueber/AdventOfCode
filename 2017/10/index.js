const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

function task1() {
  const numberInput = input.split(",").map((a) => Number(a));
  let list = [];
  for (let i = 0; i < 256; i++) list.push(i);
  let current = 0;
  let skip = 0;
  numberInput.forEach((length) => {
    const sublist = [...list, ...list].slice(current, current + length).reverse();
    for (let i = 0; i < length; i++) {
      list[(i + current) % 256] = sublist[i];
    }
    current = (current + length + skip) % 256;
    skip++;
  });

  return list[0] * list[1];
}

function task2() {
  const processedInput = [...input.split("").map((a) => a.charCodeAt(0)), 17, 31, 73, 47, 23];
  let list = [];
  for (let i = 0; i < 256; i++) list.push(i);
  let current = 0;
  let skip = 0;
  for (rounds = 0; rounds < 64; rounds++) {
    processedInput.forEach((length) => {
      const sublist = [...list, ...list].slice(current, current + length).reverse();
      for (let i = 0; i < length; i++) {
        list[(i + current) % 256] = sublist[i];
      }
      current = (current + length + skip) % 256;
      skip++;
    });
  }

  let result = "";
  for (p = 0; p < 16; p++) {
    let xor = 0;
    for (j = 0; j < 16; j++) {
      xor ^= list[p * 16 + j];
    }
    result += xor.toString(16).padStart(2, "0");
  }

  return result;
}

console.log(task1());
console.log(task2());
