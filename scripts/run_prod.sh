#!/bin/bash

# Build and run the production server.

npm ci --prefix app
npm run --prefix app build

cargo run --release
