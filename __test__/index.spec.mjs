import test from 'ava'

import { sum, httpGet } from '../index.js'

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})

test('httpGet', async (t) => {
  const res = await httpGet('https://mock.insomnia.rest')
  t.is(res, '"Hello World!"')
})


// direction
// - add methods
// - Headers
// - cookies
// - streams
// 