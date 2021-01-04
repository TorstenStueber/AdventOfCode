const { strict } = require("assert");
const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n\n")
  .map((a) => a.split("\n"));

const size = Math.sqrt(input.length);

function search(space, tiles, x, y, used) {
  if (y >= size) {
    return true;
  }

  for (let k = 0; k < input.length; k++) {
    const tile = input[k];
    const id = tile[0].split(" ")[1].split(":")[0];
    if (used.includes(id)) continue;

    const grid = tile.slice(1);
    for (let rot = 0; rot < 4; rot++) {
      for (let flip = 0; flip < 2; flip++) {
        const result = [];
        for (let y = 0; y < grid.length; y++) {
          result.push([]);
          for (let x = 0; x < grid.length; x++) {
            let xx = x;
            let yy = y;
            if (flip === 1) yy = grid.length - y - 1;
            for (j = 0; j < rot; j++) {
              const c = xx;
              xx = yy;
              yy = grid.length - c - 1;
            }
            result[y].push(grid[yy][xx]);
          }
        }

        if (x > 0) {
          let stop = false;
          for (let yy = 0; yy < grid.length; yy++) {
            if (space[y * grid.length + yy][x * grid.length - 1] !== result[yy][0]) {
              stop = true;
              break;
            }
          }
          if (stop) continue;
        }
        if (y > 0) {
          let stop = false;
          for (let xx = 0; xx < grid.length; xx++) {
            if (space[y * grid.length - 1][x * grid.length + xx] !== result[0][xx]) {
              stop = true;
              break;
            }
          }
          if (stop) continue;
        }
        for (let yy = 0; yy < grid.length; yy++) {
          for (let xx = 0; xx < grid.length; xx++) {
            if (space[y * grid.length + yy] === undefined) space[y * grid.length + yy] = [];
            space[y * grid.length + yy][x * grid.length + xx] = result[yy][xx];
          }
        }

        if (tiles[y] === undefined) tiles[y] = [];
        tiles[y][x] = id;
        if (search(space, tiles, (x + 1) % size, y + (x + 1 === size ? 1 : 0), [...used, id])) {
          return true;
        }
      }
    }
  }

  return false;
}

const tiles = [];
const result = [];
search(result, tiles, 0, 0, []);

function task1() {
  return (
    Number(tiles[0][0]) * Number(tiles[size - 1][0]) * Number(tiles[0][size - 1]) * Number(tiles[size - 1][size - 1])
  );
}

const monster = `                  # 
#    ##    ##    ###
 #  #  #  #  #  #   `.split("\n");

function task2() {
  const result2 = [];
  for (let y = 0; y < size; y++) {
    for (let yy = 0; yy < result.length / size - 2; yy++) {
      result2.push([]);
      for (let x = 0; x < size; x++) {
        for (let xx = 0; xx < result.length / size - 2; xx++) {
          result2[y * (result.length / size - 2) + yy][x * (result.length / size - 2) + xx] =
            result[(y * result.length) / size + yy + 1][(x * result.length) / size + xx + 1];
        }
      }
    }
  }

  for (let rot = 0; rot < 4; rot++) {
    for (let flip = 0; flip < 2; flip++) {
      const oriented = [];
      for (let y = 0; y < result2.length; y++) {
        oriented.push([]);
        for (let x = 0; x < result2.length; x++) {
          let xx = x;
          let yy = y;
          if (flip === 1) yy = result2.length - y - 1;
          for (j = 0; j < rot; j++) {
            const c = xx;
            xx = yy;
            yy = result2.length - c - 1;
          }
          oriented[y].push(result2[yy][xx]);
        }
      }

      const monsters = [];
      let foundMonsters = 0;
      for (let y = 0; y < oriented.length - monster.length + 1; y++) {
        for (let x = 0; x < oriented.length - monster[0].length + 1; x++) {
          let fail = false;
          for (let yy = 0; yy < monster.length && !fail; yy++) {
            for (let xx = 0; xx < monster[0].length; xx++) {
              if (monster[yy][xx] === "#" && oriented[y + yy][x + xx] !== "#") {
                fail = true;
                break;
              }
            }
          }
          if (!fail) {
            foundMonsters++;
            for (let yy = 0; yy < monster.length; yy++) {
              for (let xx = 0; xx < monster[0].length; xx++) {
                if (monster[yy][xx] === "#") monsters.push(`${x + xx};${y + yy}`);
              }
            }
          }
        }
      }

      if (foundMonsters === 0) {
        continue;
      }

      let count = 0;
      for (let y = 0; y < oriented.length; y++) {
        for (let x = 0; x < oriented.length; x++) {
          if (oriented[y][x] === "#" && !monsters.includes(`${x};${y}`)) count++;
        }
      }
      return count;
    }
  }
}

console.log(task1());
console.log(task2());
