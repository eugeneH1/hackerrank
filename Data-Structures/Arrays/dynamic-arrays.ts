function dynamicArray(n: number, queries: number[][]): number[] {
  const arr: number[][] = Array.from({ length: n }, () => []);
  let lastAnswer = 0;
  const result: number[] = [];

  for (let i = 0; i < queries.length; i++) {
    const idx = (queries[i][1] ^ lastAnswer) % n;
    if (queries[i][0] === 1) {
      arr[idx].push(queries[i][2])
    } else if (queries[i][0] === 2) {
      const index = queries[i][2] % arr[idx].length;
      lastAnswer = arr[idx][index];
      result.push(lastAnswer);
    }

  }
  return result;
}

const queries = [[1, 0, 5], [1, 1, 7], [1, 0, 3], [2, 1, 0], [2, 1, 1]];
console.log(dynamicArray(2, queries));
