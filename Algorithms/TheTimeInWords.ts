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

const minutes = new Map<number, string>([
  [0, ''],
  [10, ''],
  [20, 'twenty'],
  [30, 'thirty'],
  [40, 'twenty'],
  [50, ''],
]);

const hours = new Map<number, string>([
  [0, ''],
  [1, 'one'],
  [2, 'two'],
  [3, 'three'],
  [4, 'four'],
  [5, 'five'], // can potentially remove this
  [6, 'six'],
  [7, 'seven'],
  [8, 'eight'],
  [9, 'nine'],
  [10, 'ten'],
  [11, 'eleven'],
  [12, 'twelve']
]);

const minuteDigits = new Map<number, string>([
  [0, ''],
  [1, "one"],
  [2, "two"],
  [3, "three"],
  [4, "four"],
  [5, "five"],
  [6, "six"],
  [7, "seven"],
  [8, "eight"],
  [9, "nine"],
  [10, 'ten']
]);

const teens = new Map<number, string>([
  [11, 'eleven'],
  [12, 'twelve'],
  [13, 'thirteen'],
  [14, 'fourteen'],
  [15, 'fifteen'],
  [16, 'sixteen'],
  [17, 'seventeen'],
  [18, 'eighteen'],
  [19, 'nineteen']
]);

function timeInWords(h: number, m: number): string {
  let hour = m <= 30 ? hours.get(h % 12) : hours.get((h + 1) % 12)
  let mintueTens = minutes.get(Math.floor(m / 10) * 10)
  let mins = m < 30 ? minuteDigits.get(m % 10) : minuteDigits.get(10 - m % 10)
  if (m == 0) return `${hour} o' clock`
  if (m <= 30) {
    if (m == 30) {
      return `half past ${hour}`
    } else if (m == 15) {
      return `quarter past ${hour}`
    } else if (m == 20) {
      return `${mintueTens} minutes past ${hour}`
    } else if (m > 20) {
      return `${mintueTens} ${mins} minutes past ${hour}`
    } else if (m > 10) {
      return `${teens.get(m)} minutes past ${hour}`

    } else return `${mins} ${m == 1 ? 'minute' : 'minutes'} past ${hour}`
  } else if (m == 45) {
    return `quarter to ${hour}`
  } else if (m == 50) {
    return `ten minutes to ${hour}`
  } else if (m > 50) {
    return `${mins} minutes to ${hour}`
  } else if (m < 50 && m > 40) {
    return `${teens.get(60 - m)} minutes to ${hour}`
  } else if (m == 20) {
    return `twenty to ${hour}`
  } else return `twenty ${mins} minutes to ${hour}`

}

function main() {
  const ws: WriteStream = createWriteStream(process.env['OUTPUT_PATH']);

  const h: number = parseInt(readLine().trim(), 10);

  const m: number = parseInt(readLine().trim(), 10);

  const result: string = timeInWords(h, m);

  ws.write(result + '\n');

  ws.end();
}

