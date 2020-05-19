class Group {
  constructor () {
    this.group = []
  }

  add (value) {
    if (!this.group.includes(value)) {
      this.group.push(value)
    }
  }

  delete (value) {
    const index = this.group.indexOf(value)
    if (index > -1) {
      this.group.splice(index, 1)
    }
  }

  has (value) {
    return this.group.includes(value)
  }

  static fromIterable (iterable) {
    const newGroup = new Group()
    const iterator = iterable[Symbol.iterator]()
    let element = iterator.next()
    while (!element.done) {
      newGroup.add(element.value)
      element = iterator.next()
    }
    return newGroup
  }
}

const testGroup = new Group()
console.log(testGroup)
testGroup.add(1)
testGroup.add('soup')
testGroup.add('$')
console.log(testGroup)
console.log(`has soup?: ${testGroup.has('soup')}`)
testGroup.add(1)
testGroup.delete('soup')
testGroup.delete('soup')
console.log(testGroup)
const testArray1 = ['a', 'b', 'c']
console.log(Group.fromIterable(testArray1))
const testArray2 = []
console.log(Group.fromIterable(testArray2))
