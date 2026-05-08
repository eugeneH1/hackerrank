function reverseArray(a: number[]): number[] {
  let arr: number[] = [];
  const l = a.length;
  for (let i = 0; i < l; i++) {
    arr[i] = a[l - i - 1];
  }
  return arr;
}

const a: number[] = [1, 2, 3, 4, 5];
console.log(reverseArray(a));
