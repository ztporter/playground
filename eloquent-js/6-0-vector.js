class Vector {
  constructor (x, y) {
    this.x = x
    this.y = y
  }

  plus (vector) {
    return new Vector((this.x + vector.x), (this.y + vector.y))
  }

  minus (vector) {
    return new Vector((this.x - vector.x), (this.y - vector.y))
  }

  get length () {
    return Math.sqrt(Math.pow(this.x, 2) + Math.pow(this.y, 2))
  }
}

const testVecA = new Vector(3, 4)
const testVecB = new Vector(1, 2)

console.log(testVecA.plus(testVecB))
console.log(testVecA.minus(testVecB))
console.log(`testVecA length: ${testVecA.length}`)
