function generateRange (lowerBound, upperBound) {
  const rangeArray = []
  for (let i = lowerBound; i <= upperBound; i++) {
    rangeArray.push(i)
  }
  return rangeArray
}

function getSum (rangeArray) {
  let total = 0
  for (const element of rangeArray) {
    total += element
  }
  return total
}

console.log(getSum(generateRange(1, 10)))
