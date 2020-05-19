class MultiplicatorUnitFailure extends Error {}

function primitiveMultiply (x, y) {
  const randomNumber = Math.floor(5 * Math.random())
  if (randomNumber === 0) {
    return x * y
  } else {
    throw new MultiplicatorUnitFailure('random failure of multplicator unit')
  }
}

while (true) {
  try {
    console.log(primitiveMultiply(2, 7))
    break
  } catch (e) {
    if (e instanceof MultiplicatorUnitFailure) {
      console.log(e.message)
    } else {
      throw e
    }
  }
}
