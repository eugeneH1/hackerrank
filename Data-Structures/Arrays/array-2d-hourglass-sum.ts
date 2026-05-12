function hourGlassSum(arr: number[][]): number {
  const hourGlassLength = 3;
  const limit = arr.length - hourGlassLength + 1;
  const shape: number[][] = [[0, 0], [0, 1], [0, 2], [0, 3], [0, 4], [1, 1], [2, 0], [2, 1], [2, 2]];
  let x = 0;
  let sums: number[] = [];
  for (let i = 0; i < limit; i++) {
    for (let j = 0; j < limit; j++) {
      // sum of hourglass starting at [i][j]
      let sum: number = 0;
      for (let s = 0; s < shape.length; s++) {
        // sum += arr[i + shape[s][0]][j + shape[s][1]];
        console.log('x: ', i + shape[s][0]);
        console.log('y: ', j + shape[s][1]);
      }
      sums[x++] = sum;
    }
  }
  return Math.max(...sums);
}
const matrix: number[][] = [
  [-9, -9, -9, 1, 1, 1],
  [0, -9, 0, 4, 3, 2],
  [-9, -9, -9, 1, 2, 3],
  [0, 0, 8, 6, 6, 0],
  [0, 0, 0, -2, 0, 0],
  [0, 0, 1, 2, 4, 0]
];
console.log(hourGlassSum(matrix));
