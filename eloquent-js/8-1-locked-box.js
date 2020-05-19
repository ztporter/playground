const box = {
  locked: true,
  unlock () { this.locked = false },
  lock () { this.locked = true },
  _content: [],
  get content () {
    if (this.locked) {
      throw new Error('Locked!')
    } else {
      return this._content
    }
  }
}

function withBoxUnlocked (body) {
  const wasLocked = box.locked
  box.unlock()
  try {
    body()
  } finally {
    if (wasLocked) { box.lock() }
  }
}

withBoxUnlocked(() => { box.content.push('gold piece') })

withBoxUnlocked(() => { console.log(box.content) })

try {
  withBoxUnlocked(() => { throw new Error('Pirates!') })
} catch (e) {
  console.log('error: ', e)
}

console.log(box.locked)
