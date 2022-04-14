FROM alpine:latest

WORKDIR /rsst

COPY Cargo.toml Cargo.lock cron.sh ./
COPY src/ src/
COPY twitter/ twitter/

RUN apk add vim bash curl go rust cargo pkgconfig libressl-dev jq python3 py3-pip
RUN pip3 install tweepy
RUN cargo build --release

CMD ["bash", "-c", "./cron.sh --interval ${RSST_INTERVAL_MIN} & sleep 1; tail -f './conf/log.txt'"]

