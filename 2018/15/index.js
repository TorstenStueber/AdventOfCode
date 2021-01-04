const { readFileSync } = require("fs");
const { join } = require("path");

const input = readFileSync(join(__dirname, "input.txt"), "utf-8")
  .trim()
  .split("\n")
  .map((a) => a.split(""));

const dirs = [
  [1, 0],
  [-1, 0],
  [0, 1],
  [0, -1],
];

function initEntities(elvesAttack) {
  const entities = [];
  input.forEach((line, y) => {
    line.forEach((a, x) => {
      if (a === "E" || a === "G") {
        entities.push({ type: a, x, y, alive: true, hits: 200, attack: a === "E" ? elvesAttack : 3 });
      }
    });
  });

  return entities;
}

function identifyOrder(entities) {
  return entities
    .filter(({ alive }) => alive)
    .sort(({ x: x1, y: y1 }, { x: x2, y: y2 }) => {
      if (y1 !== y2) return y1 - y2;
      return x1 - x2;
    });
}

function flood(map, startX, startY) {
  const todo = [[startX, startY]];
  const results = {};
  results[`${startX};${startY}`] = 0;

  while (todo.length) {
    const [x, y] = todo.shift();
    const d = results[`${x};${y}`];
    dirs.forEach(([xx, yy]) => {
      if (map[y + yy][x + xx] === ".") {
        const key = `${x + xx};${y + yy}`;
        if (results[key] === undefined) {
          todo.push([x + xx, y + yy]);
          results[key] = d + 1;
        }
      }
    });
  }

  return results;
}

function executeEntityRound(e, entities, map) {
  const { type, x, y, alive, attack } = e;
  if (!alive) return;
  if (!dirs.some(([xx, yy]) => ["E", "G"].includes(map[y + yy][x + xx]) && map[y + yy][x + xx] !== type)) {
    // move
    const distancesFromEntity = flood(map, x, y);
    let min = Infinity;
    let minPoints;

    entities.forEach((entity) => {
      if (!entity.alive || entity.type === type) return;
      dirs.forEach(([xx, yy]) => {
        const key = `${entity.x + xx};${entity.y + yy}`;
        if (distancesFromEntity[key] !== undefined && distancesFromEntity[key] <= min) {
          if (distancesFromEntity[key] === min) {
            minPoints.push([entity.x + xx, entity.y + yy]);
          } else {
            min = distancesFromEntity[key];
            minPoints = [[entity.x + xx, entity.y + yy]];
          }
        }
      });
    });

    if (min === Infinity) return;

    const [goalX, goalY] = minPoints.sort(([x1, y1], [x2, y2]) => {
      if (y1 !== y2) return y1 - y2;
      return x1 - x2;
    })[0];

    const distancesFromGoal = flood(map, goalX, goalY);

    const [xx, yy] = dirs
      .filter(([xx, yy]) => distancesFromGoal[`${x + xx};${y + yy}`] !== undefined)
      .sort(([xx1, yy1], [xx2, yy2]) => {
        if (distancesFromGoal[`${x + xx1};${y + yy1}`] !== distancesFromGoal[`${x + xx2};${y + yy2}`])
          return distancesFromGoal[`${x + xx1};${y + yy1}`] - distancesFromGoal[`${x + xx2};${y + yy2}`];
        if (yy1 !== yy2) return yy1 - yy2;
        return xx1 - xx2;
      })[0];

    map[y][x] = ".";
    e.x += xx;
    e.y += yy;
    map[e.y][e.x] = type;
  }

  // Attack
  const target = entities
    .filter((entity) => {
      if (!entity.alive || entity.type === type) return false;
      const dx = Math.abs(entity.x - e.x);
      const dy = Math.abs(entity.y - e.y);
      return dx * dy === 0 && dx + dy === 1;
    })
    .sort((e1, e2) => {
      if (e1.hits !== e2.hits) return e1.hits - e2.hits;
      if (e1.y !== e2.y) return e1.y - e2.y;
      return e1.x - e2.x;
    })[0];

  if (target === undefined) return;
  target.hits -= attack;
  if (target.hits <= 0) {
    target.alive = false;
    map[target.y][target.x] = ".";
  }
}

function executeRound(entities, map) {
  const ordered = identifyOrder(entities);
  let over = false;
  ordered.forEach((entity) => {
    if (over) return;
    over =
      entities.every(({ alive, type }) => type === "E" || !alive) ||
      entities.every(({ alive, type }) => type === "G" || !alive);
    if (over) return;
    executeEntityRound(entity, entities, map);
  });

  return over;
}

function task1() {
  let rounds = 0;
  const map = [...input.map((line) => [...line])];
  const entities = initEntities(3);
  while (true) {
    const over = executeRound(entities, map);
    if (over) {
      const totalHits = entities.reduce((a, { alive, hits }) => a + (alive ? hits : 0), 0);
      return totalHits * rounds;
    }
    rounds++;
  }
}

function task2() {
  for (let elvesAttack = 3; true; elvesAttack++) {
    const map = [...input.map((line) => [...line])];
    const entities = initEntities(elvesAttack);
    let rounds = 0;
    while (true) {
      const over = executeRound(entities, map);
      if (over) {
        if (entities.every(({ alive, type }) => type === "G" || alive)) {
          const totalHits = entities.reduce((a, { alive, hits }) => a + (alive ? hits : 0), 0);
          return totalHits * rounds;
        }
        break;
      }

      rounds++;
    }
  }
}

console.log(task1());
console.log(task2());
