export function toCamelCase(str: string): string {
  let camelString: string = str;
  let i = 0;
  while (i < camelString.length) {
    if (!isAlpha(str.charAt(i))) {
      camelString =
        camelString.slice(0, i) +
        camelString.charAt(i).toUpperCase() +
        camelString.slice(i + 1, camelString.length);
    } else {
      i++;
    }
  }
  return camelString;
}
function isAlpha(char: string): boolean {
  const code = char.charCodeAt(0);
  return (code >= 65 && code <= 90) || (code >= 97 && code <= 122);
}

function toCamel(str: string): string {
  let result = "";
  for (let i = 0; i < str.length; i++) {
    if (!isAlpha(str[i])) {
      result += str[i + 1].toUpperCase();
      i++;
    } else {
      result += i;
    }
  }
  return result;
}

const tooCamel = (str: string): string => {
  const camel = str.split('')
    .reduce((acc, char, i, arr) => {
      const prevChar = arr[i - 1];

      if (!isAlpha(char)) return acc;

      if (prevChar && !isAlpha(prevChar)) {
        return acc + char.toUpperCase();
      } else {
        return acc + char.toLowerCase();
      }
    }, '');
  return camel.charAt(0).toUpperCase() + camel.slice(1);
}

console.log(tooCamel('the_stealth_warrior'));
