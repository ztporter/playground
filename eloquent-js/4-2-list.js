function arrayToList (inputArray) {
  if (inputArray.length === 0) {
    return null
  } else {
    const list = {
      value: inputArray[0],
      rest: arrayToList(inputArray.slice(1))
    }
    return list
  }
}

function listToArray (inputList) {
  if (inputList.rest === null) {
    return [inputList.value]
  } else {
    return [inputList.value, ...listToArray(inputList.rest)]
  }
}

function prepend (element, inputList) {
  return { value: element, rest: inputList }
}

function nth (inputList, index) {
  if (inputList === null) {
    return undefined
  } else if (index === 0) {
    return inputList.value
  } else {
    return nth(inputList.rest, index - 1)
  }
}

const testArray = [1, 2, 3]
const testList = arrayToList(testArray)
console.log(testList)
console.log(listToArray(testList))
console.log(JSON.stringify(prepend(4, testList)))
console.log(nth(testList, 2))
console.log(nth(testList, 0))
console.log(nth(testList, 5))
