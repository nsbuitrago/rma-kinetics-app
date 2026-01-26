import { mkdirSync, copyFileSync } from 'fs';
import { dirname } from 'path';

// Cross-platform script to copy WASM stub for Tauri builds
// This ensures the bundler can resolve the WASM import even when
// the real WASM package hasn't been built

const dest = 'src/lib/backend/wasm-pkg/rma_kinetics_wasm.js';
const src = 'src/lib/backend/wasm-stub/rma_kinetics_wasm.js';

// Create destination directory if it doesn't exist
mkdirSync(dirname(dest), { recursive: true });

// Copy stub file
copyFileSync(src, dest);

console.log('✓ Copied WASM stub to wasm-pkg/');
