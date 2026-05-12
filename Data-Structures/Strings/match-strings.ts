// function matchingStrings(stringList: string[], queries: string[]): number[] {
//   const result: number[] = [];
//   for (let i = 0; i < queries.length; i++) {
//     let count = 0;
//     stringList.forEach((str) => {
//       const reg = new RegExp(str);
//       reg.test(queries[i]) && count++;
//       result[i] = count;
//     });
//   }
//   return result;
// }

function matchingStrings(stringList: string[], queries: string[]): number[] {
  const occurrence: Record<string, number> = {};

  for (const str of stringList) {
    occurrence[str] = (occurrence[str] || 0) + 1;
  }

  return queries.map(q => occurrence[q] || 0);
}
