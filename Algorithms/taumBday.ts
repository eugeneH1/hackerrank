function taumBday(b: number, w: number, bc: number, wc: number, z: number): number {
  let blackCost = Math.min(bc, (wc + z));
  let whiteCost = Math.min(wc, (bc + z));
  return b * blackCost + w * whiteCost;
}


const black = 3; const white = 6; const blackCost = 9; const whiteCost = 1; const conversionCost = 1;
console.log(taumBday(black, white, blackCost, whiteCost, conversionCost));
