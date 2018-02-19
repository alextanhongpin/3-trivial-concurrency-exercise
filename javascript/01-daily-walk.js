async function main () {
  const names = ['Alice', 'Bob']
  console.log(`Let's go for a walk`)

  await Promise.all(names.map(gettingReady))
  alarm()
  Promise.all(names.map(puttingOnShoes)).then(_ => console.log('Exiting and locking the door'))
}

main().then().catch(console.log)

async function alarm () {
  console.log('Arming alarm')
  await randomDelayInSeconds(10000, 10000)
  console.log('Alarm is armed')
}

async function randomDelayInSeconds (min, max) {
  const delta = max - min
  const duration = min + Math.floor(Math.random() * delta)
  return delay(duration)
}

async function delay (duration) {
  return new Promise((resolve, reject) => {
    setTimeout(() => resolve(duration), duration)
  })
}

async function gettingReady (name) {
  console.log(`${name} started getting ready`)
  const duration = await randomDelayInSeconds(6000, 9000)
  console.log(`${name} spent ${duration} seconds getting ready`)
}

async function puttingOnShoes (name) {
// func puttingOnShoes(wg *sync.WaitGroup, name string) {
  console.log(`${name} started putting on shoes`)
  const duration = await randomDelayInSeconds(3000, 5000)
  console.log(`${name} spend ${duration} seconds putting on shoes`)
}
