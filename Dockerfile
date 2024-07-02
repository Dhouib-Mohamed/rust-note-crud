# Building the application
FROM rust:1.76 as build

WORKDIR /app

ARG DATABASE_URL
ARG PORT=3000

ENV DATABASE_URL=$DATABASE_URL
ENV PORT=$PORT

COPY . .

RUN cargo build --release

# Running the application
FROM debian:buster-slim

COPY --from=build /app/target/release/RustCRUD .

CMD ["./app"]