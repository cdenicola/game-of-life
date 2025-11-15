# Game of Life

A sparse HashSet implementation of Conway's Game of Life written in Rust. The project now ships with two entry points:

- `src/main.rs` – terminal preview used for quick demos.
- `web/` – WebAssembly front-end that renders the grid in a browser and lets you interact with the simulation.

## Running the web client

1. Build the WebAssembly bundle (outputs go to `web/pkg`):

   ```bash
   wasm-pack build --target web --out-dir web/pkg
   ```

2. Serve the `web` directory with any static server. For example:

   ```bash
   cd web
   python3 -m http.server 8080
   ```

3. Open `http://localhost:8080` in your browser. 
    - Use the controls to start/pause the simulation, step once, randomize the grid, adjust the tick interval, or toggle individual cells by clicking on them.
    - Scroll to zoom the viewport and hold space (or right-click drag) to pan across the infinite grid
    - Light/Dark Mode toggle lets you switch palettes at any time.

The UI talks directly to the Rust core via WebAssembly, so the same rules and tick logic power both the CLI demo and the website.

## Deployment

The `.github/workflows/pages.yml` workflow builds the WebAssembly bundle with `wasm-pack` and publishes the `web/` directory to GitHub Pages. The workflow runs on every push to `main`
