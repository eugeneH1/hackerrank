//O(n * m)
function naieveApproach(n: number, queries: number[][]): number {
  const arr: number[] = Array.from({ length: n }, () => 0);
  for (let i = 0; i < queries.length; i++) {
    for (let j = queries[i][0] - 1; j < queries[i][1]; j++) {
      arr[j] += queries[i][2];
    }
  }
  return Math.max(...arr);
}

const queries: number[][] = [
  [1, 2, 100],
  [2, 5, 100],
  [3, 4, 100]
];

//the sweep approach is O(n + m) 
//the idea is that for each query we mark the boundaries, left boundary we add k and right we subtract k
//think of the array as an elevation map, instead of marking exact elevation of every point of terrain we mark step 
//at this point elevation goes up by x and stays that way until changed
function sweepApproach(n: number, queries: number[][]): number {
  const arr: number[] = Array.from({ length: n + 2 }, () => 0);

  // O(m) - Mark the boundaries
  for (let i = 0; i < queries.length; i++) {
    const [a, b, k] = queries[i];
    arr[a] += k;
    arr[b + 1] -= k;
  }

  let max = 0;
  let currentSum = 0;

  // O(n) - Single pass to find the maximum prefix sum
  for (let i = 1; i <= n; i++) {
    currentSum += arr[i];
    if (currentSum > max) {
      max = currentSum;
    }
  }

  return max;
}
// console.log((5, queries));
