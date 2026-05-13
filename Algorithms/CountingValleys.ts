// function countingValleys(steps: number, path: string): number {
//     let valleys = 0;
//     let elevation = 0;
//     for(const char of path) {
//       switch(char) {
//         case 'U':
//           elevation += 1;
//           if(elevation === 0) {
//             valleys += 1;
//           }
//         break;
//         case 'D':
//           elevation -= 1;
//         break;
//
//     }
//   }
//   return valleys;
// }

// function countingValleys(steps: number, path: string): number {
//   let valleys = 0;
//   let elevation = 0;
//
//   for(const char of path) {
//     if(char === "U") {
//       elevation += 1;
//       if(elevation === 0) valleys += 1;
//     } else {
//       elevation -= 1;
//     }
//   }
//   return valleys;
// }

function countingValleys(steps: number, path: string): number {
  let elevation = 0;
  return path.split('').reduce((valleys, char) => {
    if (char === 'U') {
      elevation++;
      if (elevation === 0) valleys++;
    } else elevation--;
    return valleys;
  }, 0);
}
