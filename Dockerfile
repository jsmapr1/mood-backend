# https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
FROM rust:1.43 as builder

RUN USER=root cargo new --bin mood-backend
WORKDIR ./mood-backend
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

#RUN rm ./target/release/deps/mood-backend*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8088

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /mood-backend/target/release/mood-backend ${APP}/mood-backend
COPY --from=builder /mood-backend/text.json ${APP}/text.json

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./mood-backend"]
