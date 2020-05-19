function forLoop (value, test, update, body) {
  if (test(value)) {
    body(value)
    forLoop(update(value), test, update, body)
  }
}

function updateValue (value) {
  return value + 1
}

function testValue (value) {
  return value < 10
}

function bodyFunc (value) {
  console.log(value)
}

forLoop(1, testValue, updateValue, bodyFunc)
