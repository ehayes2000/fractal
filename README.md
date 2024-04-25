## Mandelbrot

A breif exploration of wasm / webgl. Bootstrapped with [wasm-pack-template](https://github.com/rustwasm/wasm-pack-template)

## Getting Started

1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
2. From the root directory `wasm-pack build` the rust package
3. From `/www` install the rust package with `npm i`
4. From `/www` use `npm run start`

## Known issues

- Flickering noise on edge of set. This either a result of vertex shader interpolation combined with fragment shader position determination and / or non-linear coloring.
