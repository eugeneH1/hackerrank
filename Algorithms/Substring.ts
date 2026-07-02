function findIndexes(str1: string, str2: string): number[] {
  let prefixLength = 0;
  let suffixLength = 0;
  let idx1 = str1.length - 1;
  let idx2 = str2.length - 1;
  while (str1.charAt(prefixLength) === str2.charAt(prefixLength)) {
    prefixLength++;
  }

  while(true) {
    if(str1.charAt(idx1) === str2.charAt(idx2)){
      idx1--;
      idx2--;
      suffixLength++;
    } else {
      break;
    }
  }
  let result = Array.from( { length: prefixLength - suffixLength + 1}, (_, index) => index + suffixLength);
  console.log(result);

  return [-0];
}

findIndexes("xyzazyx", "xyzzyx");