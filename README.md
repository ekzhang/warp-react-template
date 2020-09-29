# warp-react-template

This is a very minimal starter template that sets up React (CRA) and Warp.

## Features

- Hot reloading on both the frontend and backend in development.
- Static assets compiled and built for production.
- Minimal docker image for deployment.
- GraphQL endpoint on the server through Juniper.

This is still a work in progress. I'm trying to figure out the best way to set
up my full-stack environment before starting my next project.

## Usage

To run the development server:

```bash
# Install dependencies (only necessary once)
npm install --prefix app
cargo install systemfd cargo-watch
npm install -g concurrently

# Run the server with live reloading and proxy
scripts/run_dev.sh
```

To run the production server:

```bash
# Install dependencies, build frontend, and run the server
scripts/run_prod.sh
```

To build a minimal Docker image for the production server:

```bash
docker build .
```

The included `Dockerfile` does a multi-stage build, with the final binary
having static linking to musl libc. This means that the image is quite small:
less than 10 MB in total for the sample application.

## Environment Variables

The server looks for the following environment variables at runtime:

- `PORT`: Which port to listen on (`3535` by default).
- `RUST_LOG`: Logging directives passed to tracing.
