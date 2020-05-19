function deepEqual (a, b) {
  if (a === b) {
    return true
  } else if (a === null) {
    return false
  } else if (typeof a === 'object') {
    for (const element of Object.keys(a)) {
      if (a[element] !== b[element]) {
        return false
      }
    }
    return true
  }
}

const a = { value: 2, name: 'Phil' }
const b = { value: 2, name: 'Phil' }
const c = { value: 2, name: 'Chris' }

console.log(deepEqual(a, b))
console.log(deepEqual(a, c))
console.log(deepEqual(a.value, c.value))
console.log(deepEqual(null, undefined))
