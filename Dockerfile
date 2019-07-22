# select build image
FROM rust:1.23 as build

# create a new empty shell project
RUN USER=root cargo new --bin lunchtime
WORKDIR /my_project

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/lunchtime*
RUN cargo build --release

# our final base
FROM rust:1.23

# copy the build artifact from the build stage
COPY --from=build /target/release/lunchtime .

# set the startup command to run your binary
CMD ["./lunchtime"]