
class Menu {
  constructor () {
    const menus = ['chorizo', 'chopitos', 'pimientos de padron', 'croquetas', 'patatas bravas']
    this.dishes = sort(this.prepare(menus))
  }
  prepare (menus) {
    return menus.map(menu => {
      const plates = random(5, 10)
      return Array(plates).fill(menu)
    })
    .reduce(flatten, [])
  }
  grab () {
    if (!this.dishes.length) {
      return null
    }
    const meal = this.dishes.shift()
    return meal
  }
  hasDish () {
    return !!this.dishes.length
  }
}

class Person {
  constructor (name) {
    this.name = name
    this.isEating = false
  }

  async eat (dish) {
    this.isEating = true
    console.log(`${this.name} is enjoying ${dish}`)
    await delay(random(1000, 2000))
    this.isEating = false
  }
}

class DiningTable {
  constructor (people, menu) {
    this.people = people
    this.menu = menu
  }

  async feast () {
    if (!this.menu.hasDish()) {
      return
    }
    const freePeople = this.people
    .filter((people) => !people.isEating)

    if (!freePeople.length) {
      await delay(500)
      return this.feast()
    }

    freePeople
    .forEach(async (people) => {
      const dish = this.menu.grab()
      if (!dish) return
      await people.eat(dish)
    })

    return this.feast()
  }
}

function main () {
  const menu = new Menu()
  const people = [
    new Person('Alice'),
    new Person('Bob'),
    new Person('Charlie'),
    new Person('Dave')
  ]

  const diningTable = new DiningTable(people, menu)
  diningTable.feast()
}

main()

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

async function delay (duration) {
  return new Promise((resolve, reject) => {
    setTimeout(_ => resolve(true), duration)
  })
}
