function everythingLoop (inputArray, testFunc) {
  for (const element of inputArray) {
    if (!testFunc(element)) {
      return false
    }
    return true
  }
}

function everythingSome (inputArray, testFunc) {
  return !inputArray.some((...args) => !testFunc(...args))
}

const testArray1 = [1, 2, 3, 4]
const testArray2 = [2, 4, 6, 8]

console.log(everythingLoop(testArray1, x => (x % 2) === 0))
console.log(everythingLoop(testArray2, x => (x % 2) === 0))
console.log(everythingSome(testArray1, x => (x % 2) === 0))
console.log(everythingSome(testArray2, x => (x % 2) === 0))
