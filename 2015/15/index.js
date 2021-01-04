const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => {
    const [
      _,
      n,
      cap,
      dur,
      fla,
      tex,
      cal,
    ] = /(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)/.exec(a);
    return { n, cap: Number(cap), dur: Number(dur), fla: Number(fla), tex: Number(tex), cal: Number(cal) };
  });

function task1() {
  let max = 0;
  for (let i1 = 0; i1 <= 100; i1++) {
    for (let i2 = 0; i2 <= 100 - i1; i2++) {
      for (let i3 = 0; i3 <= 100 - i1 - i2; i3++) {
        const i4 = 100 - i1 - i2 - i3;
        const cap = Math.max(0, i1 * input[0].cap + i2 * input[1].cap + i3 * input[2].cap + i4 * input[3].cap);
        const dur = Math.max(0, i1 * input[0].dur + i2 * input[1].dur + i3 * input[2].dur + i4 * input[3].dur);
        const fla = Math.max(0, i1 * input[0].fla + i2 * input[1].fla + i3 * input[2].fla + i4 * input[3].fla);
        const tex = Math.max(0, i1 * input[0].tex + i2 * input[1].tex + i3 * input[2].tex + i4 * input[3].tex);
        max = Math.max(max, cap * dur * fla * tex);
      }
    }
  }
  return max;
}

function task2() {
  let max = 0;
  for (let i1 = 0; i1 <= 100; i1++) {
    for (let i2 = 0; i2 <= 100 - i1; i2++) {
      for (let i3 = 0; i3 <= 100 - i1 - i2; i3++) {
        const i4 = 100 - i1 - i2 - i3;
        const cap = Math.max(0, i1 * input[0].cap + i2 * input[1].cap + i3 * input[2].cap + i4 * input[3].cap);
        const dur = Math.max(0, i1 * input[0].dur + i2 * input[1].dur + i3 * input[2].dur + i4 * input[3].dur);
        const fla = Math.max(0, i1 * input[0].fla + i2 * input[1].fla + i3 * input[2].fla + i4 * input[3].fla);
        const tex = Math.max(0, i1 * input[0].tex + i2 * input[1].tex + i3 * input[2].tex + i4 * input[3].tex);
        const cal = Math.max(0, i1 * input[0].cal + i2 * input[1].cal + i3 * input[2].cal + i4 * input[3].cal);
        if (cal === 500) max = Math.max(max, cap * dur * fla * tex);
      }
    }
  }
  return max;
}

console.log(task1());
console.log(task2());
