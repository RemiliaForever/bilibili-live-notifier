FROM xd009642/tarpaulin:develop-nightly as tarpaulin

RUN apt-get update \
    && apt-get install -y libdbus-1-dev
