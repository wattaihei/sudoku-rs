FROM rust:1.43 AS builder
  
WORKDIR /sudoku

COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
COPY ./src ./src
RUN rm -f target/release/deps/sudoku*
RUN cargo build --release


FROM debian:10.4
ENV PORT=${PORT}
COPY --from=builder /sudoku/target/release/sudoku-rs /usr/local/bin/sudoku-rs
CMD ["sudoku-rs"]