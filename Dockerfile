FROM rust:slim
MAINTAINER Jan Burkhardt <264754-bjrke@users.noreply.gitlab.com>
RUN apt update
RUN apt install -y curl
RUN curl -sSfL https://deb.nodesource.com/setup_16.x | bash -
RUN apt install -y nodejs
RUN curl -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh
