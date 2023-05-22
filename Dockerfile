FROM ubuntu:latest
LABEL authors="Constantin Rohde"

RUN apt update
RUN apt -y install wget curl build-essential

COPY . .

# Install newest node version for compatability with tailwindcss
RUN curl -sL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get install nodejs

# Downloads trunk binary so we don't have to build it locally
RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- \
    && cp trunk /usr/bin

RUN npm i

# Installs rust toolchain for WASM target
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

RUN . $HOME/.cargo/env \
  && rustup target add wasm32-unknown-unknown \
  && cargo fetch

# Make rustup toolchain global, maybe there is a way to move it to PATH for the whole Dockerfile instead of copying binaries
RUN cp -r $HOME/.cargo/bin/* /usr/bin

RUN npm run build

EXPOSE 8080
ENTRYPOINT ["npm", "run", "release:docker"]
