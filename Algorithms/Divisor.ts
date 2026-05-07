
const divisor = (integer: number): number[] | string => {
  const range = Array.from({ length: integer - 2 }).map((_, i) => i + 2);
  const result = range.reduce<number[]>((acc, i) => {
    if (integer % i == 0) {
      acc.push(i);
    }
    return acc
  }, []);

  return result.length ? result : `${integer} is a prime`
}

console.log(divisor(12));
const techLead = (integer: number): number[] | string => {
  const range = Array.from({ length: integer - 2 }).map((_, i) => i + 2);
  const result = range.reduce<number[]>((acc, i) => {
    if (someCheck(i)) {
      acc.push(i);
    }

    return acc;

  }, []);

  return result.length ? result : `${integer} didn't pass the check`
}

function someCheck(num: number) {
  return true;
}
