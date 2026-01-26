# RMA Kinetics Simulator

This repository contains source code for the RMA simulator desktop applications.
There are currently four supported models including,

1. Constitutive - simple, constitutively expressed RMA
2. TetOff - tTA induced RMA expression under the Tet-Off system
3. Chemogenetic - hM3Dq + CNO activation with Tet-Off gating for monitoring neuronal activity with RMAs
4. Oscillating - RMA expression driven by an sine force function to simulate oscillating gene expression dynamics

For a detailed description of these models, see the accompanying [paper](https://www.biorxiv.org/content/10.1101/2025.11.17.688787v1.full.pdf+html) and [documentation](https://nsbuitrago.github.io/rma-kinetics-rs/docs).

## Getting Started

To install the desktop application, download the installers from the [release](https://github.com/nsbuitrago/rma-kinetics-app/releases) section for your operating
system. We support and test Windows and MacOS. If you need Linux support, please submit an issue.

## Building

This app is built with Tauri and Svelte. To develop on top of this application or build from source,
make sure that you have `cargo` and `pnpm` installed. You can build the application in development mode
with `pnpm run tauri dev`. See the [package.json](./package.json) for all available scripts.

The user interface is built with Svelte and all models are implemented in Rust using the [rma-kinetics-rs](https://github.com/nsbuitrago/rma-kinetics-rs) crate.

## Feedback

Please submit issues for feedback or bugs.
