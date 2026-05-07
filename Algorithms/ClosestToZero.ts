const closestToZero = (numbers: number[]): number => {
  let closest = Infinity;

  for (let i = 0; i < numbers.length; i++) {
    const distance = Math.abs(numbers[i]);
    const currClosestDistance = Math.abs(closest);
    if (distance < currClosestDistance) {
      closest = numbers[i];
    } else if (distance == currClosestDistance && closest > numbers[i]) {
      closest = numbers[i];

    }
  }
  return closest;
}

const closestFunction = (numbers: number[]): number => {
  return numbers.reduce((closest, curr) => {
    const absClosest = Math.abs(closest);
    const absCurr = Math.abs(curr);

    if (absCurr < absClosest) return closest = curr;

    if (absCurr === absClosest) return curr > closest ? curr : closest;

    return 0;
  }, numbers[0]);
}

console.log(closestToZero([3, 5, 2, -1, 1]));

