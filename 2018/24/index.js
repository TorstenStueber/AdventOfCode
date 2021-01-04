const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n\n")
  .map((block, blockI) =>
    block
      .split("\n")
      .slice(1)
      .map((line, lineI) => {
        const [
          _,
          units,
          hitPoints,
          specials,
          damage,
          damageType,
          initiative,
        ] = /(\d+) units each with (\d+) hit points (?:\((.*)\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)/.exec(
          line
        );

        let weak = [];
        let immune = [];
        if (specials) {
          specials.split(";").forEach((special) => {
            const [_, type, damageTypes] = /(immune|weak) to (.*)/.exec(special);
            if (type === "weak") weak = damageTypes.split(", ");
            if (type === "immune") immune = damageTypes.split(", ");
          });
        }

        const type = blockI === 0 ? "immuneSystem" : "infection";
        return {
          id: `${type}:${lineI}`,
          type,
          units: Number(units),
          hitPoints: Number(hitPoints),
          weak,
          immune,
          attackDamage: Number(damage),
          damageType,
          initiative: Number(initiative),
        };
      })
  );

function effectivePower({ units, attackDamage }) {
  return units * attackDamage;
}

function damage(attacker, defender) {
  if (defender.immune.includes(attacker.damageType)) return 0;
  if (defender.weak.includes(attacker.damageType)) return effectivePower(attacker) * 2;
  return effectivePower(attacker);
}

function fight(boost) {
  let immuneSystem = [...input[0].map((a) => ({ ...a, attackDamage: a.attackDamage + boost }))];
  let infection = [...input[1].map((a) => ({ ...a }))];
  while (immuneSystem.length > 0 && infection.length > 0) {
    // target selection
    const selectedTargets = {};
    [...immuneSystem, ...infection]
      .sort((groupA, groupB) => {
        const epA = effectivePower(groupA);
        const epB = effectivePower(groupB);
        if (epA !== epB) return epB - epA;
        return groupB.initiative - groupA.initiative;
      })
      .forEach((group) => {
        const enemyGroups = group.type === "immuneSystem" ? infection : immuneSystem;
        const selected = enemyGroups
          .filter((enemy) => !Object.values(selectedTargets).includes(enemy.id) && damage(group, enemy) > 0)
          .sort((enemyA, enemyB) => {
            const dA = damage(group, enemyA);
            const dB = damage(group, enemyB);
            if (dA !== dB) return dB - dA;
            const epA = effectivePower(enemyA);
            const epB = effectivePower(enemyB);
            if (epA !== epB) return epB - epA;
            return enemyB.initiative - enemyA.initiative;
          })[0];
        if (selected) {
          selectedTargets[group.id] = selected.id;
        }
      });

    //attack
    let attackHappened = false;
    [...immuneSystem, ...infection]
      .sort((groupA, groupB) => {
        return groupB.initiative - groupA.initiative;
      })
      .forEach((group) => {
        if (selectedTargets[group.id] === undefined) return;
        if (group.units <= 0) return;

        const enemy = [...immuneSystem, ...infection].find((g) => g.id === selectedTargets[group.id]);
        const d = damage(group, enemy);
        const kills = Math.floor(d / enemy.hitPoints);
        if (kills > 0) attackHappened = true;
        enemy.units -= kills;
        if (enemy.units <= 0) {
          immuneSystem = immuneSystem.filter((g) => g.id !== enemy.id);
          infection = infection.filter((g) => g.id !== enemy.id);
        }
      });

    if (!attackHappened) break;
  }

  return { immuneSystem, infection };
}

function task1() {
  const { immuneSystem, infection } = fight(0);
  return immuneSystem.reduce((a, g) => a + g.units, 0) + infection.reduce((a, g) => a + g.units, 0);
}

function task2() {
  let min = 0;
  let max;
  for (let i = 1; true; i *= 2) {
    const { infection } = fight(i);
    if (infection.length === 0) {
      max = i;
      break;
    }
  }

  while (min + 1 !== max) {
    let mid = Math.floor((min + max) / 2);
    const { infection } = fight(mid);
    if (infection.length === 0) {
      max = mid;
    } else {
      min = mid;
    }
  }

  const { immuneSystem } = fight(max);
  return immuneSystem.reduce((a, g) => a + g.units, 0);
}
console.log(task1());
console.log(task2());
