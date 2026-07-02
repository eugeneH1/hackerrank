function minimumNumber(n: number, password: string): number {
  const numRegex = /\d/;
  const lowerCaseRegex = /[a-z]/;
  const upperCaseRegex = /[A-Z]/;
  const specialCharsRegex = /[!@#$%^&*()\-+_]/;

  let result = 0;
  result += numRegex.test(password) ? 0 : 1;
  console.log(numRegex.test(password));
  result += lowerCaseRegex.test(password) ? 0 : 1;
  console.log(lowerCaseRegex.test(password));
  result += upperCaseRegex.test(password) ? 0 : 1;
  console.log(upperCaseRegex.test(password));
  result += specialCharsRegex.test(password) ? 0 : 1;
  console.log(specialCharsRegex.test(password));

  return result;
}

console.log(minimumNumber(3, "Ab1"));
