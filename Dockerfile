FROM rust:latest AS builder
WORKDIR /usr/src/myarchinstall
COPY . .
RUN cargo install --path .

FROM archlinux:latest
COPY --from=builder /usr/local/cargo/bin/myarchinstall /usr/local/bin/myarchinstall
CMD ["myarchinstall"]