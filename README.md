# Vinting

## About

Vinting is a REST api school project that uses a relational database and has a responsive frontend.

Features:

- [x] Relational database
- [x] Rest api
- [x] Responsive frontend

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
NO_WEB=1 cargo build
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

### Frontend:

Run the backend and then start the vite development server in the `vinting-web` directory with `npm run dev`

### Backend:

The server is runnable in debug mode without any extra data outside of the repo.
When running in release mode there should be a `.env` or a `.env.production` file with a few environment variables

Available environment variables:

- JWT_SECRET: **MANDATORY FOR PRODUCTION**, gets set to `secret` in debug mode
- ADMIN_EMAIL: sets the email for which an admin role will be registered
- INMEMORY: **DEBUG EXCLISIVE**, isn't read through the .env file, runs the database in the memory instead of a file

Build environment variables:

- NPM: sets the executable with which to execute the npm commands
- REBUILD: force rebuild the client
- NO_WEB: build the server without building the client

> [!NOTE]
> See the [endpoints](./ENDPOINTS.md) documentation for details on endpoints

## Repo file structure (excluding the client)

- src: Source files for the server
- dtos: This houses the dtos and the helper types used in the dtos (e.g. EmailString)
- entity: This has the sea-orm models and helper functions applied to sea-orm generated types (currently only has ActiveAction)
- services: Definitions for services and the service trait
- migrations: Migrations for the db, currently only has seed data
