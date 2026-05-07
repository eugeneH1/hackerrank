function taumBday(b: number, w: number, bc: number, wc: number, z: number): number {
  let cost = 0;
  if (bc + z < wc) {
    cost += b * bc;
    cost += w * (bc + z);
  } else if (wc + z < bc) {
    cost += w * wc;
    cost += b * (wc + z);
  } else {
    cost += wc * w + bc * b;
  }
  return cost;
}


const black = 3; const white = 6; const blackCost = 9; const whiteCost = 1; const conversionCost = 1;
console.log(taumBday(black, white, blackCost, whiteCost, conversionCost));
