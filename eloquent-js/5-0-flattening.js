function flatten (inputArray) {
  return inputArray.reduce((a, b) => a.concat(b))
}

const testArray = [[1, 2, 3], [4, 5], [6]]

console.log(flatten(testArray))
