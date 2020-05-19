const fizzes = [
  ['fizz', 3],
  ['buzz', 5]
]

function printFizz (input) {
  let output = ''

  for (let i = 0; i < fizzes.length; i++) {
    if ((input % fizzes[i][1]) === 0) {
      output += fizzes[i][0]
    }
  }

  if (output === '') {
    output = input.toString()
  }

  console.log(output)
}

function fizzBuzz (startValue, stopValue) {
  for (let i = startValue; i <= stopValue; i++) {
    printFizz(i)
  }
}

const startValue = 1
const stopValue = 100

fizzBuzz(startValue, stopValue)
