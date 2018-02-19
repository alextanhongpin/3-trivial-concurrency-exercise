
async function main () {
  const menus = ['chorizo', 'chopitos', 'pimientos de padron', 'croquetas', 'patatas bravas']
  const people = ['Alice', 'Bob', 'Charlie', 'Dave']
  const state = people.reduce((l, r) => {
    l[r] = false
    return l
  }, {})
  const dishes = sort(prepare(menus))

  const feast = async (people, dishes) => {
    if (!dishes.length) {
      return
    }
    const allEating = Object.values(state).every(i => i)
    if (allEating) {
      // console.log('heartbeat...')
      await delay(random(150, 300))
    }
    people.forEach(async(person) => {
      if (state[person]) return
      state[person] = true
      const dish = dishes.shift()
      if (!dish) return
      console.log(`${person} is enjoying ${dish}`)

      await delay(random(1000, 2000))
      state[person] = false
    })

    return feast(people, dishes)
  }
  console.log('Bon appetit!')
  await feast(people, dishes)
  console.log('That was delicious!')
}

main().then().catch(console.error)

function flatten (l = [], r = []) {
  return l.concat(r)
}

function random (min = 0, max = 10) {
  const delta = max - min
  const value = min + Math.floor(Math.random() * delta)
  return value
}

function sort (arr) {
  return arr
  .map(i => [i, Math.random()])
  .sort(([_, a], [__, b]) => a > b)
  .map(([a, _]) => a)
}

function prepare (menus) {
  return menus.map(menu => {
    const plates = random(5, 10)
    return Array(plates).fill(menu)
  })
  .reduce(flatten, [])
}

async function delay (duration) {
  return new Promise((resolve, reject) => {
    setTimeout(_ => resolve(true), duration)
  })
}
