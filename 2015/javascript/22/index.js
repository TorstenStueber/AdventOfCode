const { readFileSync, chmod } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => Number(a.split(":")[1]));

const boss = { hit: input[0], damage: input[1] };

function task1() {
  let minSpent = Infinity;
  function game(youHits, mana, spells, bossHits, player, spentMana) {
    // depth limit dfs
    if (spentMana > 10000) return;

    function end(winner) {
      if (winner === "you") minSpent = Math.min(minSpent, spentMana);
    }

    if (bossHits <= 0) {
      end("you");
      return;
    }

    if (youHits <= 0) {
      end("boss");
      return;
    }

    if (spells.shield > 0) {
      spells = { ...spells, shield: spells.shield - 1 };
    }
    if (spells.poison > 0) {
      bossHits -= 3;
      if (bossHits <= 0) {
        end("you");
        return;
      }
      spells = { ...spells, poison: spells.poison - 1 };
    }
    if (spells.recharge > 0) {
      mana += 101;
      spells = { ...spells, recharge: spells.recharge - 1 };
    }

    if (player === "you") {
      if (mana >= 53) {
        game(youHits, mana - 53, spells, bossHits - 4, "boss", spentMana + 53);
      }
      if (mana >= 73) {
        game(youHits + 2, mana - 73, spells, bossHits - 2, "boss", spentMana + 73);
      }
      if (mana >= 113 && !(spells.shield > 0)) {
        game(youHits, mana - 113, { ...spells, shield: 6 }, bossHits, "boss", spentMana + 113);
      }
      if (mana >= 173 && !(spells.poison > 0)) {
        game(youHits, mana - 173, { ...spells, poison: 6 }, bossHits, "boss", spentMana + 173);
      }
      if (mana >= 229 && !(spells.recharge > 0)) {
        game(youHits, mana - 229, { ...spells, recharge: 5 }, bossHits, "boss", spentMana + 229);
      }
    } else {
      game(youHits - Math.max(1, boss.damage - (spells.shield > 0 ? 7 : 0)), mana, spells, bossHits, "you", spentMana);
    }
  }

  game(50, 500, {}, boss.hit, "you", 0);

  return minSpent;
}

function task2() {
  let minSpent = Infinity;
  function game(youHits, mana, spells, bossHits, player, spentMana) {
    // depth limit dfs
    if (spentMana > 1000000) return;

    function end(winner) {
      if (winner === "you") minSpent = Math.min(minSpent, spentMana);
    }

    if (bossHits <= 0) {
      end("you");
      return;
    }

    if (player === "you") youHits--;

    if (youHits <= 0) {
      end("boss");
      return;
    }

    if (spells.shield > 0) {
      spells = { ...spells, shield: spells.shield - 1 };
    }
    if (spells.poison > 0) {
      bossHits -= 3;
      if (bossHits <= 0) {
        end("you");
        return;
      }
      spells = { ...spells, poison: spells.poison - 1 };
    }
    if (spells.recharge > 0) {
      mana += 101;
      spells = { ...spells, recharge: spells.recharge - 1 };
    }

    if (player === "you") {
      if (mana >= 53) {
        game(youHits, mana - 53, spells, bossHits - 4, "boss", spentMana + 53);
      }
      if (mana >= 73) {
        game(youHits + 2, mana - 73, spells, bossHits - 2, "boss", spentMana + 73);
      }
      if (mana >= 113 && !(spells.shield > 0)) {
        game(youHits, mana - 113, { ...spells, shield: 6 }, bossHits, "boss", spentMana + 113);
      }
      if (mana >= 173 && !(spells.poison > 0)) {
        game(youHits, mana - 173, { ...spells, poison: 6 }, bossHits, "boss", spentMana + 173);
      }
      if (mana >= 229 && !(spells.recharge > 0)) {
        game(youHits, mana - 229, { ...spells, recharge: 5 }, bossHits, "boss", spentMana + 229);
      }
    } else {
      game(youHits - Math.max(1, boss.damage - (spells.shield > 0 ? 7 : 0)), mana, spells, bossHits, "you", spentMana);
    }
  }

  game(50, 500, {}, boss.hit, "you", 0);

  return minSpent;
}

console.log(task1());
console.log(task2());
