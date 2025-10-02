/// <reference types="@vitest/browser/providers/playwright" />

import { expect, test } from 'vitest'
import { add } from 'com-gm112-rust-testlibrary'

test('adds 1 + 2 to equal 3', () => {
  const result = add(1, 2)
  expect(result).toBe(3)
})
