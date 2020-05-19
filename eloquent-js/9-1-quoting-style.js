const singleQuoteRegex = /(\W|^)'([^]+?)'(\W|$)/

const testQuote = `'The curfew tolls the knell of parting day,
    The lowing herd wind slowly o'er the lea,
The plowman homeward plods his weary way,
    And leaves the world to darkness and to me.'\n
                            - Thomas Gray`

console.log(testQuote.replace(singleQuoteRegex, '"$2"'))
