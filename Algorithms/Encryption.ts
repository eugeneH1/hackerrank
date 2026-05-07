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
 * Complete the 'encryption' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

function encryption(s: string): string {
  let block: string[] = [];
  let encrypted: string = '';
  const strippedStr = s.replace(/\s+/g, '');
  let rows = Math.floor(Math.sqrt(strippedStr.length));
  const cols = Math.sqrt(strippedStr.length) == rows ? rows : rows + 1;
  if ((rows * cols) < strippedStr.length) {
    rows = cols
  }
  // let lastStringLenght = (rows * cols) - strippedStr.length;
  let lastStringLenght = strippedStr.length % cols
  let count = 0;
  for (let i = 0; i < rows; i++) {
    block[i] = strippedStr.substring(count, count + cols)
    count = count + cols;
  }
  for (let i = 0; i < cols; i++) {
    let limit = 0
    if (lastStringLenght == 0) {
      limit = rows
    } else {
      limit = i >= lastStringLenght ? rows - 1 : rows
    }

    for (let j = 0; j < limit; j++) {
      if (block[j][i] == undefined) continue;
      encrypted += block[j][i]
    }
    encrypted += ' '
  }
  return encrypted

}

function main() {
  const ws: WriteStream = createWriteStream(process.env['OUTPUT_PATH']);

  const s: string = readLine();

  const result: string = encryption(s);

  ws.write(result + '\n');

  ws.end();
}

