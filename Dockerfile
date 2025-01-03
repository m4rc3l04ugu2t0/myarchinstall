FROM rust:latest AS builder
WORKDIR /usr/src/myarchinstall
COPY . .
RUN cargo install --path .

FROM archlinux:latest
COPY setup.toml /var/lib/archinstall/config.toml
ENV CONFIG_PATH=/var/lib/archinstall/config.toml
RUN pacman -Syu --noconfirm
COPY --from=builder /usr/local/cargo/bin/myarchinstall /usr/local/bin/myarchinstall
CMD ["myarchinstall"]