async function main () {
  const tourists = Array(25)
    .fill(0)
    .map((_, i) => [i + 1, Math.random()])
    .sort(([_, a], [__, b]) => a > b)
    .map(([a, _]) => a)

  const maxConcurrency = 8
  await concurrentOnline(tourists, goOnline, maxConcurrency)
  console.log(`The place is empty, let's close up and go to the beach!`)
}

async function goOnline (i) {
  console.log(`Tourist ${i} is online.`)
  const duration = await delay(random(1000, 2000))
  console.log(`Tourist ${i} is done, having spent ${duration} seconds online.`)
  return i
}

function random (min, max) {
  const delta = max - min
  return min + Math.floor(Math.random() * delta)
}

async function delay (duration) {
  return new Promise((resolve, reject) => {
    setTimeout(_ => resolve(duration), duration)
  })
}

main().then().catch(console.error)

async function concurrentOnline (arr, fn, maxConcurrency = 10, concurrency = 0, iter = 0) {
  if (!arr.length) return
  let pending = []

  arr.map(async (item) => {
    const isLimit = concurrency >= maxConcurrency
    if (isLimit) {
      pending.push(item)
      return
    }
    concurrency += 1
    await fn(item)
    concurrency -= 1
  })

  if (iter === 0) {
    pending.forEach(i => console.log(`Tourist ${i} is waiting for turn.`))
  }

  while (concurrency === maxConcurrency) {
    await delay(random(150, 300))
  }
  return concurrentOnline(pending, fn, maxConcurrency, concurrency, iter + 1)
}
