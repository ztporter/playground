function reverseArray (inputArray) {
  const reversedArray = []
  for (const element of inputArray) {
    reversedArray.unshift(element)
  }
  return reversedArray
}

function reverseArrayInPlace (inputArray) {
  let frontIndex = 0
  let backIndex = inputArray.length - 1
  while (frontIndex < backIndex) {
    const temp = inputArray[frontIndex]
    inputArray[frontIndex] = inputArray[backIndex]
    inputArray[backIndex] = temp
    frontIndex++
    backIndex--
  }
}

const testArray = ['alex', 'bruce', 'clyde', 'daniel']

console.log(reverseArray(testArray))
console.log(testArray)
reverseArrayInPlace(testArray)
console.log(testArray)
