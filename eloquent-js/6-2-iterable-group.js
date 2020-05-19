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

  [Symbol.iterator] () {
    return new GroupIterator(this.group)
  }
}

class GroupIterator {
  constructor (group) {
    this.index = 0
    this.group = group
  }

  next () {
    if (this.index < this.group.length) {
      const element = { value: this.group[this.index], done: false }
      this.index++
      return element
    } else {
      return { value: undefined, done: true }
    }
  }
}

const testGroup = new Group()
testGroup.add('a')
testGroup.add('b')
testGroup.add('c')
for (const element of testGroup) {
  console.log(element)
}
