FROM rust:1.69 as wasm-build

ENV PATH=$PATH:~/.cargo/bin

# This could be more reproducible by pinning the version of wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

# Cargo updates the cargo.io index whenever it runs. To avoid this we can cache the index
# Caching for cargo: https://stackoverflow.com/questions/67001033/skip-updating-crates-io-index-when-using-cargo-run
# Caching in gitlab: https://docs.gitlab.com/ee/ci/caching/

RUN wasm-pack build --target bundler --release --out-name index --out-dir ./pkg
RUN cargo doc --no-deps

FROM node:latest as frontend-build

ENV CI=true
ARG API_URL=/api/v1

WORKDIR /app

COPY --from=wasm-build /app/pkg /app/pkg

COPY package.json package.json
COPY package-lock.json package-lock.json
COPY webpack.config.js webpack.config.js
COPY tsconfig.json tsconfig.json

COPY ./ts ./ts
COPY ./static ./static
COPY ./bindings ./bindings

RUN npm install && npm run frontend-build

# FROM nginx:latest

# COPY --from=frontend-build /app/dist /usr/share/nginx/html

# EXPOSE 80

FROM alpine:3.12

# Image Shrinking effect
# - activate to perform all steps ending in one image layer
# - deactivate all RUN commands above
RUN apk update && apk add nginx && rm -rf /var/cache/apk/*

# ADD web /var/www/html
COPY --from=frontend-build /app/dist /var/www/html
COPY --from=wasm-build /app/target/doc /var/www/html/doc
ADD nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

ENTRYPOINT ["nginx", "-g", "daemon off;pid /tmp/nginx.pid;"]

#(100 / 132) * 6.99 = 5.30 %
# 100 - 5.30 = 94.70 % kleiner
