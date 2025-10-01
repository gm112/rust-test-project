# Rust Test Project

This is a test project for Rust.

## Setup

```bash
rustup toolchain install
rustup target add wasm32-unknown-unknown
npm install -g corepack@latest

corepack enable
pnpm install

pnpm exec playwright install
```

## Commands

The root pnpm workspace implements several commands to orchestrate the project. It interacts with both the TypeScript and Rust projects.

### pnpm playground

```bash
pnpm playground
```

### pnpm test

```bash
pnpm test
```

### pnpm format

```bash
pnpm format
```

### pnpm lint

```bash
pnpm lint
```

### pnpm build

```bash
pnpm build
```

### pnpm build:prod

```bash
pnpm build:prod
```

## License

[MPL-2.0](./LICENSE)
