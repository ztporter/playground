class Map {
  hasOwnProperty (value) {
    return `how should I know ${value}?`
  }
}

const myMap = new Map()
myMap.cats = 'meow'
console.log(myMap.hasOwnProperty('cats'))
console.log(Object.prototype.hasOwnProperty.call(myMap, 'cats'))
