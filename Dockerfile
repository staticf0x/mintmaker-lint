FROM rust:1.81 as builder

WORKDIR /usr/src/mintmaker-lint
COPY . .
RUN cargo install --path .

FROM registry.access.redhat.com/ubi9-minimal

COPY --from=builder /usr/local/cargo/bin/mintmaker-lint /usr/local/bin/mintmaker-lint
CMD ["mintmaker-lint"]
