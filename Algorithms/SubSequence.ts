function subSeq(S: string): string {
  if (S.length === 0) return "";
  const first = S.charAt(0);
  const rest = subSeq(S.slice(1));
  let result = "";
  for (const subStr of rest.split(",")) {
    result += ", " + first + subStr;
    result += ", " + subStr;
  }
  return result.slice(1);
}

const subSeq2 = (S: string): string => {
  if (S === "") return "";
  return S + ", " + subSeq2(S.slice(1)) + ", " + subSeq2(S.slice(1, 3));
}
console.log(subSeq2("abc"));

function findSubsets(S: string): void {

  // Helper function that tracks our current position and the current subset built so far
  function helper(index: number, currentSubset: string): void {
    // Base Case: If we've made a decision for every character, print it and stop
    if (index === S.length) {
      console.log(`"${currentSubset}"`);
      return;
    }

    // 'a' index = 0
    const nextChar = S.charAt(index);

    // Branch 1: INCLUDE the current character and move forward
    // cs = "" 
    helper(index + 1, currentSubset + nextChar);

    // Branch 2: EXCLUDE the current character and move forward
    helper(index + 1, currentSubset);
  }

  // Start the recursion at index 0 with an empty subset string
  helper(0, "");
}

findSubsets("abc");
