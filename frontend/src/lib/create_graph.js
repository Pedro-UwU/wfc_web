const EMPTY = -1;

// Grid must be a 2D array of ints
export const create_graph = (grid) => {
    let graph = {};
    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[0].length; j++) {
            let value = grid[i][j];
            if (value === EMPTY) {
                continue;
            }
            if (!graph[value]) {
                graph[value] = {
                    "n": [],
                    "e": [],
                    "s": [],
                    "w": []
                };
            }
            // TODO: Check neighbors
            if (i > 0 && grid[i - 1][j] !== EMPTY) {
                graph[value]["n"].push(grid[i-1][j]);
            }

            if (i < grid.length - 1 && grid[i + 1][j] !== EMPTY) {
                graph[value]["s"].push(grid[i+1][j]);
            }

            if (j > 0 && grid[i][j - 1] !== EMPTY) {
                graph[value]["w"].push(grid[i][j-1]);
            }

            if (j < grid[0].length - 1 && grid[i][j + 1] !== EMPTY) {
                graph[value]["e"].push(grid[i][j+1]);
            }

        };
    }
    return graph;
}
