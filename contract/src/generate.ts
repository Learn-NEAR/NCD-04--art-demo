import { near } from "near-sdk-js";

const SIZE = 22;
const HALF_SIZE = SIZE / 2;

const schema = ["🟣", "🟡️️", "⚫️", "⭕️", "🔘"];

export function generate(seed: number): string {
  let output = "";

  const a = seed == 0 ? Number(randomNum()) : seed;

  if (seed === 0) {
    near.log(
      `\n\n\tCall claimMyDesign with the seed number ${a} to claim it.\n`
    );
  }

  const mod = (a % 11) + 5;

  for (let i = 0; i < SIZE; i++) {
    let y = 2 * (i - HALF_SIZE) + 1;

    if (a % 3 === 1) {
      y *= -1;
    } else if (a % 3 === 2) {
      y = abs(y);
    }

    y *= a;

    for (let j = 0; j < SIZE; j++) {
      let x = 2 * (j - HALF_SIZE) + 1;

      if (a % 2 === 1) {
        x = abs(x);
      }

      x *= a;

      const v = abs(x * y) % mod;

      const value = v < 5 ? schema[v] : "⚪️";

      output += value;
    }

    output += "\n";
  }

  return output;
}

function abs(num: number): number {
  return num >= 0 ? num : -num;
}

function randomNum(): number {
  const randomSeed = near.randomSeed();

  let sum = BigInt(0);

  for (let index = 0; index < randomSeed.length; index++) {
    sum += BigInt(randomSeed.charCodeAt(index));
  }

  return Number(sum % BigInt(near.blockHeight().toString()));
}
