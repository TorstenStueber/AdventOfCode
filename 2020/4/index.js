const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8").trim().split("\n\n");

function task1() {
  let count = 0;
  input.forEach((p) => {
    p = p.trim().split("\n").join(" ");

    let byr = false;
    let iyr = false;
    let eyr = false;
    let hgt = false;
    let hcl = false;
    let ecl = false;
    let pid = false;

    p.split(" ").forEach((l) => {
      const s = l.split(":")[0];
      if (s === "byr") byr = true;
      if (s === "iyr") iyr = true;
      if (s === "eyr") eyr = true;
      if (s === "hgt") hgt = true;
      if (s === "hcl") hcl = true;
      if (s === "ecl") ecl = true;
      if (s === "pid") pid = true;
    });

    if (byr && iyr && eyr && hgt && hcl && ecl && pid) count++;
  });

  return count;
}

function task2() {
  let count = 0;
  input.forEach((p) => {
    p = p.trim().split("\n").join(" ");

    let byr = false;
    let iyr = false;
    let eyr = false;
    let hgt = false;
    let hcl = false;
    let ecl = false;
    let pid = false;

    p.split(" ").forEach((l) => {
      const s = l.split(":")[0];
      const v = l.split(":")[1];
      if (s === "byr") {
        if (v.length === 4 && Number(v) >= 1920 && Number(v) <= 2002) byr = true;
      }
      if (s === "iyr") {
        if (v.length === 4 && Number(v) >= 2010 && Number(v) <= 2020) iyr = true;
      }
      if (s === "eyr") {
        if (v.length === 4 && Number(v) >= 2020 && Number(v) <= 2030) eyr = true;
      }
      if (s === "hgt") {
        const m = /^(\d+)(cm|in)$/.exec(v);
        if (
          m &&
          ((m[2] === "cm" && Number(m[1]) >= 150 && Number(m[1]) <= 193) ||
            (m[2] === "in" && Number(m[1]) >= 59 && Number(m[1]) <= 76))
        )
          hgt = true;
      }
      if (s === "hcl") {
        const m = /^#[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]$/.exec(v);
        if (m) hcl = true;
      }
      if (s === "ecl") {
        if (["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].indexOf(v) >= 0) ecl = true;
      }
      if (s === "pid") {
        const m = /^[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]$/.exec(v);
        if (m) pid = true;
      }
    });

    if (byr && iyr && eyr && hgt && hcl && ecl && pid) count++;
  });

  return count;
}

console.log(task1());
console.log(task2());
