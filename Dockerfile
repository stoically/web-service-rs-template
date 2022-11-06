FROM debian:bullseye-slim AS builder

ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt -y update
RUN apt -y upgrade
RUN apt -y install build-essential curl libssl-dev cmake pkg-config
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN mkdir /build
WORKDIR /build
COPY . /build
RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo build --release
RUN --mount=type=cache,target=/build/target \
    mv /build/target/release/template-web-service .

FROM debian:bullseye-slim
COPY --from=builder /build/template-web-service /opt/template-web-service
EXPOSE 8080/tcp
CMD [ "/opt/template-web-service", "spawn" ]