import { defineConfig } from 'vitest/config'
import wasm from 'vite-plugin-wasm'
export default defineConfig({
  plugins: [wasm()],
  test: {
    include: ['./app/**/*.node.test.ts'],
    exclude: ['./app/**/*.browser.test.ts'],
    environment: 'node',
    globals: true,
  },
})
