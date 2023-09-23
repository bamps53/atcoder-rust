FROM rust
RUN cargo install cargo-compete \
&& rustup install 1.42.0 \
&& rustup component add rustfmt --toolchain 1.42.0