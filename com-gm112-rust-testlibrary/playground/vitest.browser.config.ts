import { defineConfig } from 'vitest/config'
import wasm from 'vite-plugin-wasm'
import { playwright } from '@vitest/browser-playwright'

export default defineConfig({
  plugins: [wasm()],
  test: {
    include: ['./app/**/*.browser.test.ts'],
    exclude: ['./app/**/*.node.test.ts'],
    browser: {
      provider: playwright(),
      enabled: true,
      headless: true,
      // at least one instance is required
      instances: [{ browser: 'chromium' }],
    },
  },
})
