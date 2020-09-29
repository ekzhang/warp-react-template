#!/bin/bash

# Run the development server, with live reloading for frontend and backend.

# Requires `npm install` to have been run in the app/ folder, as well as the
# following programs to be installed and present on your path:
#
# - systemfd (`cargo install systemfd`)
# - cargo-watch (`cargo install cargo-watch`)
# - concurrently (`npm install -g concurrently`)

export RUST_LOG=info

concurrently -n frontend,backend -c blue,red \
    "npm run --prefix app start" \
    "systemfd --color always --no-pid -s http::3535 \
        -- cargo watch -x 'run --color always'"
