const squareValues = [' ', '#']

function buildRow (startIndex, width) {
  let row = '|'
  let index = startIndex
  for (let i = 0; i < width; i++) {
    row += squareValues[index]
    index = (index + 1) % squareValues.length
  }
  row += '|\n'
  return row
}

function drawBoard (width, height) {
  let output = '-'.repeat(width + 2) + '\n'
  let index = 0
  for (let i = 0; i < height; i++) {
    output += buildRow(index, width)
    index = (index + 1) % squareValues.length
  }
  output += '-'.repeat(width + 2)
  console.log(output)
}

const width = 17
const height = 9
drawBoard(width, height)
