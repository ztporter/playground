function countChar (inputString, searchChar) {
  let count = 0
  for (let i = 0; i < inputString.length; i++) {
    if (inputString[i] === searchChar) {
      count++
    }
  }
  return count
}

function countB (inputString) {
  return countChar(inputString, 'B')
}

const testString = 'Bristol Beats Barcelona in Bocce Ball'

console.log(countB(testString))
console.log(countChar(testString, 'l'))
