# diff-match-patch-wasm

## About

This is a WebAssembly wrapper around the [diff-match-patch-rs](https://crates.io/crates/diff-match-patch-rs) library. It is
intended to be used in the browser.

## Installation

```bash
npm install diff-match-patch-wasm
```

## Usage

```javascript
import { Differ } from 'diff-match-patch-wasm';

const dmp = new Differ();
const diffs = dmp.diff_main('Hello, world!', 'Goodbye, world!');
console.log(diffs);
```

## Development

To build the WebAssembly module, run:

```bash
wasm-pack build
```

To run the tests, run:

```bash
wasm-pack test --chrome --headless
```
