const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim();

const width = 25;
const height = 6;

function task1() {
  const layers = [];
  for (let start = 0; start < input.length; start += width * height) {
    layers.push(input.slice(start, start + width * height));
  }

  let minIndex = 0;
  let min = Infinity;
  layers.forEach((layer, i) => {
    const no = layer.split("").filter((a) => a === "0").length;
    if (no < min) {
      min = no;
      minIndex = i;
    }
  });

  const layer = layers[minIndex];
  return layer.split("").filter((a) => a === "1").length * layer.split("").filter((a) => a === "2").length;
}

function task2() {
  const layers = [];
  for (let start = 0; start < input.length; start += width * height) {
    layers.push(input.slice(start, start + width * height));
  }

  let result = undefined;
  layers.forEach((layer) => {
    if (!result) {
      result = layer;
      return;
    }
    result = result
      .split("")
      .map((r, i) => (r === "2" ? layer[i] : r))
      .join("");
  });

  for (let line = 0; line < width * height; line += width) {
    console.log(
      result
        .slice(line, line + width)
        .replace(/0/g, " ")
        .replace(/1/g, "*")
    );
  }
}

console.log(task1());
console.log(task2());
