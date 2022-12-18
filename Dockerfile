FROM docker.io/rust:slim-bullseye as builder
ARG CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN apt update -yqq \
	&& apt install -yqq --no-install-recommends \
	build-essential cmake libssl-dev pkg-config git \
	&& rustup update \
	&& rustup toolchain add stable \
	&& rustup default stable

COPY . /app
WORKDIR /app
RUN cargo build --release


FROM debian:bullseye-slim
RUN apt update && apt install -y ca-certificates
RUN mkdir -p /opt/app
WORKDIR /opt/app

COPY --from=builder /app/target/release/gameshow-v2 /usr/local/bin/gameshow-v2
COPY --from=builder /app/questions /opt/app/questions
COPY --from=builder /app/static /opt/app/static

ENV RUST_BACKTRACE=1
ENV BIND_ADDRESS="0.0.0.0:8000"
ENV MAX_NICKNAME_LENGTH=25

CMD ["/usr/local/bin/gameshow-v2"]
