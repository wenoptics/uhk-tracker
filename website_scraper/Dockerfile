FROM rust:1-buster as builder

# Create a new empty shell project
RUN cargo new --bin website_scraper
WORKDIR /website_scraper

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release && rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/website_scraper*
RUN cargo build --release


FROM rust:1-buster
COPY --from=builder /website_scraper/target/release/website_scraper .

CMD ["./website_scraper"]
