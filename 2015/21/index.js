const { readFileSync, chmod } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => Number(a.split(":")[1]));

const boss = { hit: input[0], damage: input[1], armor: input[2] };

const shop = `Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3`;

const weapons = shop.split("\n\n")[0].split("\n").slice(1).map(parseLine);
const armors = shop.split("\n\n")[1].split("\n").slice(1).map(parseLine);
const rings = shop.split("\n\n")[2].split("\n").slice(1).map(parseLine);

function parseLine(line) {
  const splits = line.split(/\s+/);
  return {
    cost: Number(splits[splits.length - 3]),
    damage: Number(splits[splits.length - 2]),
    armor: Number(splits[splits.length - 1]),
  };
}

function game(you, boss) {
  while (true) {
    boss.hit -= Math.max(you.damage - boss.armor, 1);
    if (boss.hit <= 0) return "you";
    you.hit -= Math.max(boss.damage - you.armor, 1);
    if (you.hit <= 0) return "boss";
  }
}

function task1() {
  let min = Infinity;
  weapons.forEach((weapon) => {
    function fight(cost, damage, armor) {
      if (game({ hit: 100, damage, armor }, { ...boss }) === "you") min = Math.min(min, cost);
    }

    function pickRings(cost, damage, armor) {
      fight(cost, damage, armor);
      rings.forEach((ring) => {
        fight(cost + ring.cost, damage + ring.damage, armor + ring.armor);
      });

      rings.forEach((ring1, i1) => {
        rings.forEach((ring2, i2) => {
          if (i1 < i2) {
            fight(
              cost + ring1.cost + ring2.cost,
              damage + ring1.damage + ring2.damage,
              armor + ring1.armor + ring2.armor
            );
          }
        });
      });
    }

    pickRings(weapon.cost, weapon.damage, weapon.armor);
    armors.forEach((armor) => {
      pickRings(weapon.cost + armor.cost, weapon.damage + armor.damage, weapon.armor + armor.armor);
    });
  });

  return min;
}

function task2() {
  let max = 0;
  weapons.forEach((weapon) => {
    function fight(cost, damage, armor) {
      if (game({ hit: 100, damage, armor }, { ...boss }) === "boss") max = Math.max(max, cost);
    }

    function pickRings(cost, damage, armor) {
      fight(cost, damage, armor);
      rings.forEach((ring) => {
        fight(cost + ring.cost, damage + ring.damage, armor + ring.armor);
      });

      rings.forEach((ring1, i1) => {
        rings.forEach((ring2, i2) => {
          if (i1 < i2) {
            fight(
              cost + ring1.cost + ring2.cost,
              damage + ring1.damage + ring2.damage,
              armor + ring1.armor + ring2.armor
            );
          }
        });
      });
    }

    pickRings(weapon.cost, weapon.damage, weapon.armor);
    armors.forEach((armor) => {
      pickRings(weapon.cost + armor.cost, weapon.damage + armor.damage, weapon.armor + armor.armor);
    });
  });

  return max;
}

console.log(task1());
console.log(task2());
