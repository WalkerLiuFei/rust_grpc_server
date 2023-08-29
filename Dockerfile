# Build Stage
FROM rust:latest AS build

WORKDIR /app

RUN git clone https://github.com/WalkerLiuFei/rust_grpc_server.git .

WORKDIR /app/rust_grpc_server

# check to the dev branch, and
RUN git checkout dev
# Build the dependencies
RUN cargo build --release

# Final Stage of example-service.
FROM scratch AS example-service

WORKDIR /app

# Copy only the compiled binary from the build stage
COPY --from=build /app/app/target/release/example-service .

# Expose the gRPC server port
EXPOSE 9090

# Start the application
CMD ["./example-service"]


# Final Stage of gateway.
FROM scratch AS gateway

WORKDIR /app

# Copy only the compiled binary from the build stage
COPY --from=build /app/app/target/release/gateway .

# Expose the HTTP port
EXPOSE 3000

# Start the application
CMD ["./gateway"]
