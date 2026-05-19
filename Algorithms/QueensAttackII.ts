// function queensAttack(n: number, k: number, r_q: number, c_q: number, obstacles: number[][]): number {
//   let board = Array.from({ length: n }, () => Array.from({ length: n }, () => 0));
//   obstacles.forEach(([row, col]) => board[row - 1][col - 1] = 1);
//
//   const moves = [[1, 0], [0, -1], [-1, 0], [0, 1], [1, 1], [1, -1], [-1, -1], [-1, 1]]
//   let count = 0;
//   for (let i = 0; i < 8; i++) {
//     let x = r_q - 1;
//     let y = c_q - 1;
//
//     while (x < n && y < n && x >= 0 && y >= 0) {
//       x += moves[i][0];
//       y += moves[i][1];
//       if (atEdge(x, y, n) || board[x][y] === 1) {
//         break;
//       } else {
//         count++;
//       }
//     }
//   }
//   return count;
// }
//
// function atEdge(x: number, y: number, n: number): boolean {
//   return !(x < n && y < n && x >= 0 && y >= 0)
// }
// Define a tuple for [x, y] coordinates for better precision than just number[]

type Vector2 = [x: number, y: number];

interface DirectionalMeta {
  v_u: Vector2 | null;   // Vertical Up
  d_r_u: Vector2 | null; // Diagonal Right Up
  h_r: Vector2 | null;   // Horizontal Right
  d_r_d: Vector2 | null; // Diagonal Right Down
  v_d: Vector2 | null;   // Vertical Down
  d_l_d: Vector2 | null; // Diagonal Left Down
  h_l: Vector2 | null;   // Horizontal Left
  d_l_u: Vector2 | null; // Diagonal Left Up
}
function queensAttack(n: number, k: number, r_q: number, c_q: number, obstacles: number[][]): number {
  const meta = Object.values(filterObstacles(obstacles, r_q, c_q))
  const edgeDistances: number[] = [
    n - r_q,                        // v_u
    Math.min(n - r_q, n - c_q),     // d_r_u
    n - c_q,                        // h_r
    Math.min(r_q - 1, n - c_q),     // d_r_d
    r_q - 1,                        // v_d
    Math.min(r_q - 1, c_q - 1),     // d_l_d
    c_q - 1,                        // h_l
    Math.min(n - r_q, c_q - 1)      // d_l_u
  ];
  let count = 0;
  for (let i = 0; i < 8; i++) {
    if (!meta[i]) {
      count += edgeDistances[i]
    } else {
      const [r, c] = meta[i]
      const delta = Math.max(Math.abs(r - r_q), Math.abs(c - c_q))
      count += delta - 1;
    }
  }
  return count;
}

function filterObstacles(obstacles: number[][], r_q: number, c_q: number): DirectionalMeta {
  // let writeIndex = 0;
  const meta: DirectionalMeta = {
    v_u: null, d_r_u: null, h_r: null, d_r_d: null,
    v_d: null, d_l_d: null, h_l: null, d_l_u: null
  };

  for (let readIndex = 0; readIndex < obstacles.length; readIndex++) {
    const [r, c] = obstacles[readIndex]
    const rowDelta = Math.abs(r_q - r)
    const colDelta = Math.abs(c_q - c)
    // Horizontal Movement (Same Row)
    if (r_q === r) {
      if (c > c_q) {
        meta.h_r = [r, c] as Vector2; // Horizontal Right
      } else if (c < c_q) {
        meta.h_l = [r, c] as Vector2; // Horizontal Left
      }
    }
    // Vertical Movement (Same Column)
    else if (c_q === c) {
      if (r > r_q) {
        meta.v_u = [r, c] as Vector2; // Vertical Up
      } else if (r < r_q) {
        meta.v_d = [r, c] as Vector2; // Vertical Down
      }
    }
    // Diagonal Movement
    else if (rowDelta === colDelta) {
      if (r > r_q) { // Row increases -> Up
        if (c > c_q) {
          meta.d_r_u = [r, c] as Vector2; // Right Up
        } else if (c < c_q) {
          meta.d_l_u = [r, c] as Vector2; // Left Up
        }
      } else if (r < r_q) { // Row decreases -> Down
        if (c > c_q) {
          meta.d_r_d = [r, c] as Vector2; // Right Down
        } else if (c < c_q) {
          meta.d_l_d = [r, c] as Vector2; // Left Down
        }
      }
    }
    // obstacles.length = writeIndex
  }
  return meta;
}

function AbsoluteDistance(x1: number, x2: number): number {
  return Math.abs(x1 - x2)

}
function atEdge(x: number, y: number, n: number): boolean {
  return !(x < n && y < n && x >= 0 && y >= 0)
}
