const CHARACTERS = 26;

function charToIndex(c) {
  return c.charCodeAt(0) - "A".charCodeAt(0);
}

function indexToChar(i) {
  return String.fromCharCode(i + "A".charCodeAt(0));
}

function generateReverseRotor(rotor) {
  let reverseRotor = new Array(CHARACTERS);
  for (let i = 0; i < CHARACTERS; i++) {
    reverseRotor[rotor.charCodeAt(i) - "A".charCodeAt(0)] = i;
  }

  let reverseRotorString = "";
  for (let i = 0; i < CHARACTERS; i++) {
    reverseRotorString += indexToChar(reverseRotor[i]);
  }

  return reverseRotorString.toLowerCase();
}

// rotor from cli arguments
const ROTOR = process.argv[2].toUpperCase();
const REVERSE_ROTOR = generateReverseRotor(ROTOR);

console.log(REVERSE_ROTOR);
