
# Rust Web Launcher

A multi‑app Rust backend, compiling to WASM frontend and Rocket as the server launcher. The repository contains a Rocket launcher, multiple WASM apps under `apps/`, and a modular build and mount structure. I've imported Leptos for the frontend but this can be easily replaced with your favourite frontend.

## How it works

Each app lives under:
```sh
apps/<app-name>/
```
and using Trunk, builds into:
```sh
apps/<app-name>/dist/
```
The launcher then loops through and serves these dist/ folders using Rocket, to make each app live on their relevant path. For the homepage this is '/', for app1 this is '/app1' and so on.

# How to Install

## Dependencies
Install required tools: 
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
cargo install trunk
```
Clone the repository:
```sh
git clone https://github.com/CGEE47/rust-multi-app-weblauncher.git
cd rust-multi-app-weblauncher
```
## Build the home app
The home app is sent to "/", so it uses the default public URL:
```sh
cd apps/home
trunk build
cd ../..
```
## Build all other apps
Every other app must be built with a --public-url matching its mount path.
Use this template:
```sh
v=app1; cd apps/$v
trunk build --public-url /$v
cd ../..
```
Replace 'app1' with the name of the app you want to build.
## Running the launcher
After building the apps:
```sh
cargo run -p launcher
```
