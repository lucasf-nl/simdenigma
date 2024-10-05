// The cli argument contains a 26 character long array of characters.
// This program converts them to c chars, subtracts 'a' from each, and prints the array.
let alphabet = process.argv[2].split("");
let alphabetArray = [];
for (let i = 0; i < alphabet.length; i++) {
  alphabetArray[i] = alphabet[i].charCodeAt(0) - "a".charCodeAt(0);
}

console.log(alphabetArray);
