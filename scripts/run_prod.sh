#!/bin/bash

# Build and run the production server. Useful for testing, but deployments
# should generally go through the `Dockerfile` instead.

npm ci --prefix app
npm run --prefix app build

cargo run --release
