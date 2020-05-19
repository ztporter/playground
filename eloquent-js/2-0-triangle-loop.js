function printTriangle (sideLength) {
  for (let i = 1; i <= sideLength; i += 1) {
    console.log('#'.repeat(i))
  }
}

const testValue = 7

printTriangle(testValue)
