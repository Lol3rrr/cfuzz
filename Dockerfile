FROM node:16 as web_builder

WORKDIR /usr/src/website

COPY ./dashboard .

RUN npm install
RUN npm run build

FROM rust:1.60-buster as fuzzer_builder

WORKDIR /usr/src/fuzzer

COPY ./fuzzer .

RUN cargo build --release

FROM debian:buster-slim

ARG APP=/usr/src/app

RUN mkdir -p ${APP}
RUN mkdir -p ${APP}/assets

COPY --from=fuzzer_builder /usr/src/fuzzer/target/release/cfuzz ${APP}/cfuzz
COPY --from=web_builder /usr/src/website/public/ ${APP}/assets/

RUN ls ${APP}/assets

WORKDIR ${APP}

ENTRYPOINT ["./cfuzz"]