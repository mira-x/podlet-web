FROM --platform=$BUILDPLATFORM docker.io/library/rust:1 AS chef
WORKDIR /app
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
RUN ["/bin/bash", "-c", "set -o pipefail && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash"]
RUN cargo binstall -y cargo-chef
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
  "linux/amd64") echo x86_64-unknown-linux-musl > /rust_target.txt ;; \
  "linux/arm64/v8") echo aarch64-unknown-linux-musl > /rust_target.txt && \
     apt update && apt install -y gcc-aarch64-linux-gnu ;; \
  *) exit 1 ;; \
esac
RUN rustup target add $(cat /rust_target.txt)

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook \
--profile dist \
--target $(cat /rust_target.txt) \
--recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY src ./src 
RUN cargo build \
--profile dist \
--target $(cat /rust_target.txt)
RUN cp target/$(cat /rust_target.txt)/dist/podlet .

FROM scratch
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80
EXPOSE 80
LABEL org.opencontainers.image.source="https://github.com/mira-x/podlet-web"
LABEL org.opencontainers.image.description="Generate Podman Quadlet files from docker-compose files on the web"
LABEL org.opencontainers.image.licenses="MPL-2.0"
COPY --from=builder /app/podlet /usr/local/bin/
ENTRYPOINT [ "/usr/local/bin/podlet" ]
