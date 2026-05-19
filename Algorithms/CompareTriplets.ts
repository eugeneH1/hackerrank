function compareTriplets(a: number[], b: number[]): number[] {
  return a.reduce((scores, aScore, i, aArr) => {
    if (aScore > b[i]) scores[0] += 1;
    if (aScore < b[i]) scores[1] += 1;
    return scores;
  }, [0, 0])
}

function compareTrips(a: number[], b: number[]): number[] {
  return a.reduce((s, v, i) => [s[0] + +(v > b[i]), s[1] + +(b[i] > v)], [0, 0]);
}
