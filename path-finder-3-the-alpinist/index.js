const DIRECTION = [
    [0, 1], // up
    [0, -1], // down
    [-1, 0], // left
    [1, 0], // right
];

class Position {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
}

class PriorityQueue {
    constructor() {
        this.items = [];
    }

    enqueue(item, cost) {
        this.items.push({ pos: item, cost });
        this.items.sort((a, b) => a.cost - b.cost);
    }

    dequeue() {
        return this.items.shift();
    }

    isEmpty() {
        return this.items.length === 0;
    }
}


/**
 * You are at start location [0, 0] in mountain area of NxN and you can only move in one of the four cardinal
 * directions (i.e. North, East, South, West). Return minimal number of climb rounds to target location [N-1, N-1].
 * Number of climb rounds between adjacent locations is defined as difference of location altitudes (ascending or descending).
 */
class Maze {
    width = 0;
    height = 0;
    grid = [];
    start = new Position(0, 0);
    end = null;
    costs = null;


    getPathHeight(pos) {
        // console.debug("getPathHeight", pos, this.grid[pos.y][pos.x]);//, this.costs);
        return this.grid[pos.y][pos.x];
    }

    getCost(pos) {
        // console.debug("getCost", pos);//, this.costs);
        return this.costs[pos.y][pos.x];
    }

    setCost(pos, cost) {
        this.costs[pos.y][pos.x] = cost;
    }

    /*
    backtrack(end, start) {
        const { x: startX, y: startY } = start;
        let curr = end;
        let climbs = [];

        while (curr.x !== startX || curr.y !== startY) {
            let climb = this.getPathHeight(curr)
            let { x: currX, y: currY } = curr;

            for (const [dx, dy] of DIRECTION) {
                const next = new Position(currX + dx, currY + dy);
                const { x: nextX, y: nextY } = next;


                if (nextX < 0 || nextY < 0 || nextY >= this.height || nextX >= this.width) {
                    continue;
                }
                console.debug("climbs vs costs", climbs, this.getCost(next), this.getCost(curr));
                const currH = this.getPathHeight(curr);
                const nextH = this.getPathHeight(next);
                console.debug("path", curr, next, currH, nextH);

                console.debug("yolo", climbs, currH, nextH);
                climbs.push(climb + Math.abs(currH - nextH));
                if (this.getCost(next) == this.getCost(curr) - 1) {
                    curr = next;
                    // break;
                }
            }
        }
        console.table(climbs);
        return climbs;
    }*/

    solve2() {
        const start = [this.start.x, this.start.y];
        const end = [this.end.x, this.end.y];

        const paths = [];
        const visited = new Set();
        const maze = this.grid;
        const width = this.width;
        const height = this.height;

        function dfs(current, path) {
            visited.add(`${current[0]},${current[1]}`);
            if (current[0] === end[0] && current[1] === end[1]) {
                paths.push(path);
            } else {
                const row = current[0];
                const col = current[1];
                const neighbors = [
                    [row + 1, col],
                    [row - 1, col],
                    [row, col + 1],
                    [row, col - 1],
                ];
                for (let [r, c] of neighbors) {
                    if (
                        r >= 0 &&
                        r < height &&
                        c >= 0 &&
                        c < width &&
                        true &&
                        !visited.has(`${r},${c}`)
                    ) {
                        dfs([r, c], [...path, [r, c]]);
                    }
                }
            }
            visited.delete(`${current[0]},${current[1]}`);
        }

        dfs(start, [start]);

        const costs = [];
        console.log("maze", maze);
        for (let path of paths) {
            let cost = 0;
            let front = path.shift();
            let lastHeight = this.getPathHeight({ x: front[0], y: front[1] });
            for (const [x, y] of path) {
                const diff = Math.abs(lastHeight - this.getPathHeight({ x, y }));
                cost += diff;
                lastHeight = this.getPathHeight({ x, y });
            }
            costs.push(cost);
        }
        return Math.min(...costs);
    }
/*
    solve() {
        let visited = new Set();
        let queue = new PriorityQueue();

        queue.enqueue(this.start, 0);
        let finished = [];

        while (!queue.isEmpty()) {
            let curr = queue.dequeue();
            const { x, y } = curr.pos;
            // if (x == this.end.x && y == this.end.y) {
            // console.log(curr);
            // this.backtrack()
            // TODO: backtrace
            // }

            if (visited.has(`${x}x${y}`)) continue;
            visited.add(`${x}x${y}`);

            for (const [dx, dy] of DIRECTION) {
                const next_row = y + dy;
                const next_col = x + dx;
                if (next_row < 0 || next_row >= this.height) continue;
                if (next_col < 0 || next_col >= this.width) continue;

                const next = new Position(next_col, next_row);
                const currH = this.getPathHeight(curr.pos);
                const nextH = this.getPathHeight(next);
                const newCost = this.getCost(curr.pos) + Math.abs(currH - nextH) + 1;

                // console.debug("new cost", newCost, this.getCost(next));
                if (newCost <= this.getCost(next)) {
                    this.setCost(next, newCost);
                    queue.enqueue(next, newCost);
                }
            }
        }

        console.table(this.costs);

        // return 42;

        return this.backtrack(this.end, this.start);

        // if (finished.length == 0) return -1;

        // console.debug(costs)
        // return 42;//Math.min(...finished);
    }
*/
    static fromString(str) {
        const maze = new Maze();
        const lines = str.split('\n');
        maze.height = lines.length;
        maze.width = lines[0].length;
        maze.end = new Position(maze.width - 1, maze.height - 1);
        maze.start = new Position(0, 0);
        // maze.costs = new Array(maze.height).fill().map(() => new Array(maze.width).fill(Infinity));
        // maze.setCost(maze.start, 0);

        for (const line of lines) {
            let row = [];
            for (const ch of line.trim()) {
                row.push(Number(ch));
            }
            maze.grid.push(row);
        }

        return maze;
    }
}

function pathFinder(area) {
    const m = Maze.fromString(area);
    const solve = m.solve2();
    // console.debug(solve);
    return solve;
}

/// ------------------- ///
///      Tests          ///
/// ------------------- ///
const { assert } = require('chai');

function testArea(expected, area) {
    let actual = pathFinder(area);
    assert.strictEqual(actual, expected, area);
}

describe("Basic tests", () => {
      it("Flat lands", () =>
         testArea(0,
    `000
    000
    000`));

    it("Wall", () =>
        testArea(2,
            `010
    010
    010`));

          it("Checkerboard", () =>
            testArea(4,
        `010
        101
        010`));

          it("Checkerboard 2", () =>
            testArea(42,
        `0707
        7070
        0707
        7070`));

          it("Massif Central", () =>
            testArea(14,
        `700000
        077770
        077770
        077770
        077770
        000007`));

    //     it("Crest path", () =>
    //         testArea(0,
    //             `777000
    // 007000
    // 007000
    // 007000
    // 007000
    // 007777`));

    //       it("Minor obstacles", () =>
    //         testArea(4,
    //     `000000
    //     000000
    //     000000
    //     000010
    //     000109
    //     001010`));
});
