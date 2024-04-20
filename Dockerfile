FROM rust:1.77

WORKDIR /usr/src/shalombot4
COPY . .

RUN cargo install --path .

CMD ["shalombot4"]
