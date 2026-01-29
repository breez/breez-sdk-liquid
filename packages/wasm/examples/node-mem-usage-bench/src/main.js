const fs = require('fs')
const path = require('path')
const v8 = require('v8')

const { connect, defaultConfig } = require('@breeztech/breez-sdk-liquid/node')

// Error handling configuration
const SILENT_ERRORS = process.env.SILENT_ERRORS === 'true'

process.on('uncaughtException', (err) => {
  if (!SILENT_ERRORS) {
    console.error('Uncaught Exception:', err.message)
    console.error(err.stack)
  }
})

process.on('unhandledRejection', (reason, promise) => {
  if (!SILENT_ERRORS) {
    console.error('Unhandled Rejection at:', promise, 'reason:', reason)
  }
})

const ROUNDS = parseInt(process.argv[2] || '20', 10) // test depth
const INSTANCES = parseInt(process.argv[3] || '120', 10) // width per round
const SHUTDOWN_GRACE_MS = 2_000 // gives grace period for shutdown
const SNAPDIR = path.join(__dirname, 'snapshots')
const API_KEY = process.env.BREEZ_API_KEY
if (!API_KEY) {
  console.error('BREEZ_API_KEY is not set')
  process.exit(1)
}

if (!global.gc) {
  console.error('\nMust run with  --expose-gc\n')
  process.exit(1)
}
if (!fs.existsSync(SNAPDIR)) fs.mkdirSync(SNAPDIR, { recursive: true })

/* helpers */

function memNow() {
  const m = process.memoryUsage()
  return { ts: Date.now(), heap: m.heapUsed, ext: m.external, rss: m.rss }
}
function mb(bytes) {
  return (bytes / 1_048_576).toFixed(1)
}
function printMem(tag, m) {
  console.log(`${tag.padEnd(15)}  heap=${mb(m.heap)} MB  ext=${mb(m.ext)} MB  rss=${mb(m.rss)} MB`)
}
function delay(ms) {
  return new Promise((r) => setTimeout(r, ms))
}

function writeSnapshot(label) {
  return new Promise((resolve) => {
    const file = fs.createWriteStream(path.join(SNAPDIR, `${label}-${Date.now()}.heapsnapshot`))
    v8.getHeapSnapshot().pipe(file).on('finish', resolve)
  })
}

/* test round */

async function createAndDestroyRound(roundIdx) {
  const instances = []
  const listeners = []
  const listenerIds = []

  /* 1. create */
  const createPromises = Array.from({ length: INSTANCES }, async (_, i) => {
    const cfg = defaultConfig('mainnet', API_KEY)
    cfg.workingDir = `./breez/memory-test-${roundIdx}-${i}`

    const seed = [...new Uint8Array(32).fill((roundIdx + i) & 0xff)]

    try {
      const sdk = await connect({ config: cfg, seed })
      const noop = () => {}
      const listenerId = await sdk.addEventListener(noop)
      return { sdk, listener: noop, listenerId, index: i }
    } catch (e) {
      console.error(`Failed to create instance ${i} in round ${roundIdx}:`, e.message)
      if (!SILENT_ERRORS) {
        console.error('Stack trace:', e.stack)
      }
      return null
    }
  })

  const results = await Promise.all(createPromises)
  
  // Filter out failed instances and populate arrays
  results.forEach(result => {
    if (result) {
      instances.push(result.sdk)
      listeners.push(result.listener)
      listenerIds.push(result.listenerId)
    }
  })

  /* 2. tear‑down */
  // Remove event listeners in parallel
  const removeListenerPromises = instances.map((instance, i) => 
    instance.removeEventListener(listenerIds[i]).catch((err) => {
      console.error(`Failed to remove event listener ${i} in round ${roundIdx}:`, err.message)
    })
  )
  await Promise.all(removeListenerPromises)

  // Disconnect instances in parallel
  const disconnectPromises = instances.map((instance, i) => 
    instance.disconnect().catch((err) => {
      console.error(`Failed to disconnect instance ${i} in round ${roundIdx}:`, err.message)
    })
  )
  await Promise.all(disconnectPromises)

  // give some time to shutdown
  await delay(SHUTDOWN_GRACE_MS)

  // break JS references properly
  instances.splice(0, instances.length)
  listeners.splice(0, listeners.length)
  listenerIds.splice(0, listenerIds.length)

  /* 3. collect */
  global.gc()

  await delay(10000) // take some time to collect

  const m = memNow()
  printMem(`after round ${roundIdx + 1}`, m)
  return m
}

/* main */

const main = async () => {
  console.log('\n' + '='.repeat(80))
  console.log(`  MEMORY TEST: ${ROUNDS} rounds × ${INSTANCES} instances per round`)
  console.log(`  Silent errors: ${SILENT_ERRORS ? 'ON' : 'OFF'}`)
  console.log('='.repeat(80))
  const initial = memNow()
  printMem('Initial', initial)
  console.log('Writing initial heap snapshot...')
  await writeSnapshot('initial')
  console.log('')

  const series = []

  for (let r = 0; r < ROUNDS; r++) {
    process.stdout.write(`Round ${(r + 1).toString().padStart(2)}/${ROUNDS} ... `)
    try {
      const roundResult = await createAndDestroyRound(r)
      series.push(roundResult)
      
      if (r === 0 || r === ROUNDS - 1) {
        console.log(`Writing heap snapshot for round ${r}...`)
        await writeSnapshot(`round-${r}`)
      }
    } catch (e) {
      console.log(`✗ FAILED: ${e.message}`)
      if (!SILENT_ERRORS) {
        console.error('Stack trace:', e.stack)
      }
    }
  }

  /* summary */
  const first = series[0]
  const last = series[series.length - 1]
  const heapChange = mb(last.heap - first.heap)
  const externalChange = mb(last.ext - first.ext)
  const rssChange = mb(last.rss - first.rss)

  console.log('\n' + '='.repeat(80))
  console.log('  SUMMARY')
  console.log('='.repeat(80))
  
  if (series.length === 0) {
    console.log('No successful rounds completed!')
    return
  }

  console.log(`Successful rounds: ${series.length}/${ROUNDS}`)
  console.log('')
  console.log(`Memory changes (first → last successful round):`)
  console.log(`  Heap:     ${heapChange.padStart(8)} MB`)
  console.log(`  External: ${externalChange.padStart(8)} MB`)
  console.log(`  RSS:      ${rssChange.padStart(8)} MB`)
  console.log('='.repeat(80) + '\n')
}

main()