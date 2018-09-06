FROM rustlang/rust:nightly

RUN apt-get update \
    && apt-get install -y libdbus-1-dev
