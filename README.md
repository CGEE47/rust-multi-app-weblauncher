
# Astrel

A multi‑app Rust backend, compiling to WASM frontend and Rocket as the server launcher. The repository contains a Rocket launcher, multiple WASM apps under `apps/`, and a modular build and mount structure.

## Dependencies

Clone the repository:
```sh
git  clone <repo-url>
cd  astrel
```
Install  required  tooling: 
```sh
rustup  update
rustup  target  add  wasm32-unknown-unknown
cargo  install  trunk
```
## Building  the  Apps

Each  app  lives  under:
```sh
apps/<app-name>/
```
and  builds  into:
```sh
apps/<app-name>/dist/
```
The  launcher  serves  these  dist/  folders  directly.

## Build  the  home  app
The  home  app  is  served  at  /,  so  it  uses  the  default  public  URL:
```sh
cd  apps/home && trunk  build && cd  ../..
```
##  Build  all  other  apps
Every  other  app  must  be  built  with  a  --public-url  matching  its  mount  path.
Use  this  pattern:
```sh
v=app1; cd  apps/$v && trunk  build  --public-url  /$v && cd  ../..
```
Replace  app1  with  the  name  of  the  app  you  want  to  build.
## Running  the  Launcher
After  building  the  apps:
```sh
cargo  run  -p  launcher
```