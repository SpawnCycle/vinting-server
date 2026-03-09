# Vinting

## About

Vinting is a REST api school project that uses a relational database and has a responsive frontend.

Features:

- [x] Relational database
- [ ] Rest api
- [ ] Responsive frontend (In progress)

## Prerequisites

- [Node.js](https://nodejs.org/en/download) and npm (or npm compatible node package managers like [pnpm](https://pnpm.io/installation) or [bun](https://bun.com/docs/installation))
- [Rust toolchain](https://rust-lang.org/tools/install/)

## Cloning the repo

> [!WARNING]
> This project utilizes git submodules, if you don't clone the submodules you won't be able to compile the app

```sh
git clone https://github.com/SpawnCycle/vinting-server.git --recursive
```

## Building the app

```sh
cargo build
```

Or if you want to build it in release mode

```sh
cargo build --release
```

Or if you want to build with a different npm compatible package manager (eg. bun)

```sh
NPM=bun cargo build
```

Or if you want to rebuild the client webapp

```sh
REBUILD=1 cargo build
```

Or if you don't want to build the client at all

```sh
REBUILD=1 cargo build
```

## Running the app

```sh
cargo run
```

Or if you want to run it in release mode

```sh
cargo run --release
```

Or if you want to run with a different npm compatible package manager (eg. bun)

```sh
NPM=bun cargo run
```

## Development

- Backend: Nothing special, just rerun/rebuild/retest the app when you're done making changes
- Frontend: Run the backend and then start vite in the `vinting-web` directory with `npm run dev`

> [!NOTE]
> See the [endpoints](./ENDPOINTS.md) documentation for details on endpoints

## Repo file structure (excluding the client)

- dtos: This houses the dtos and the helper types used in the dtos (e.g. EmailString)
- entity: This has the sea-orm models and helper functions applied to sea-orm generated types (currently only has ActiveAction)
- services: Definitions for services and the service trait
- src: Source files for the server
