// function getMoneySpent(keyboards, drives, b) {
//     let max = -1;
//     for(let i = 0; i < keyboards.length; i++) {
//         for(let j = 0; j < drives.length; j++) {
//             const sum = keyboards[i] + drives[j];
//             if(sum <= b && sum > max) max = sum;
//         }
//     }
//     return max;
// }

function getMoneySpent(keyboards, drives, b) {
  let max = -1;

  const sortedK = keyboards.sort((a, b) => b - a);
  const sortedD = drives.sort((a, b) => a - b);

  let i = 0
  let j = 0

  while (i < sortedK.length && j < sortedD.length) {
    let sum = sortedK[i] + sortedD[j];
    if (sum > b) {
      i++;
    } else {
      //within budget
      if (sum > max) max = sum;
      j++
    }
  }
  return max;
}
