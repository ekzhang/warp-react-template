FROM ekidd/rust-musl-builder as backend
WORKDIR /home/rust/src
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
COPY src src
COPY sqlx-data.json sqlx-data.json
ENV SQLX_OFFLINE 1
RUN touch src/main.rs
RUN cargo test --release
RUN cargo build --release

FROM node:alpine as frontend
WORKDIR /usr/src/app
COPY app/package.json app/package-lock.json ./
RUN npm ci
COPY app .
RUN npm run build

FROM scratch
COPY --from=frontend /usr/src/app/build app/build
COPY --from=backend /home/rust/src/target/x86_64-unknown-linux-musl/release/server .
USER 1000:1000
CMD [ "./server" ]
