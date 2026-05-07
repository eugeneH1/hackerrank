'use strict';

import { WriteStream, createWriteStream } from "fs";
process.stdin.resume();
process.stdin.setEncoding('utf-8');

let inputString: string = '';
let inputLines: string[] = [];
let currentLine: number = 0;

process.stdin.on('data', function (inputStdin: string): void {
  inputString += inputStdin;
});

process.stdin.on('end', function (): void {
  inputLines = inputString.split('\n');
  inputString = '';

  main();
});

function readLine(): string {
  return inputLines[currentLine++];
}

/*
 * Complete the 'nonDivisibleSubset' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY s
 */

function nonDivisibleSubset(k: number, s: number[]): number {
  const remainderCount: number[] = Array(k).fill(0);

  // Count frequency of each remainder
  for (const num of s) {
    remainderCount[num % k]++;
  }

  // Start with at most one element with remainder 0
  let maxSubsetSize = Math.min(remainderCount[0], 1);

  // Handle the pairs of remainders
  for (let r = 1; r <= Math.floor(k / 2); r++) {
    if (r === k - r) {
      // Special case when remainder is exactly half of k
      maxSubsetSize += Math.min(remainderCount[r], 1);
    } else {
      maxSubsetSize += Math.max(remainderCount[r], remainderCount[k - r]);
    }
  }

  return maxSubsetSize;

}

function main() {
  const ws: WriteStream = createWriteStream(process.env['OUTPUT_PATH']);

  const firstMultipleInput: string[] = readLine().replace(/\s+$/g, '').split(' ');

  const n: number = parseInt(firstMultipleInput[0], 10);

  const k: number = parseInt(firstMultipleInput[1], 10);

  const s: number[] = readLine().replace(/\s+$/g, '').split(' ').map(sTemp => parseInt(sTemp, 10));

  const result: number = nonDivisibleSubset(k, s);

  ws.write(result + '\n');

  ws.end();
}

