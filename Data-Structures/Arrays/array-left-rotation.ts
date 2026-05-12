function rotate(d: number, arr: number[]): number[] {
  let res: number[] = [];
  const inverse = arr.length - d;

  for (let i = 0; i < arr.length; i++) {
    let index = i;
    if (i < d) {
      index += inverse;
      res[index] = arr[i];
    } else {
      index -= d;
      res[index] = arr[i];
    }
  }
  return res;
}
const ex1 = [1, 2, 3, 4, 5];
console.log(rotate(2, ex1))
