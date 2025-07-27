FROM rust:latest AS builder

WORKDIR /usr/src/helloWorld
COPY . .

RUN cargo install --path .

#CMD ["helloWorld"]

################################################################################
# Create a new stage for running the application that contains the minimal
################################################################################

FROM amazonlinux:2023 AS final

# Copy the executable from the "build" stage.
#COPY --from=build /bin/helloWorld /bin/
COPY --from=builder /usr/local/cargo/bin/helloWorld /usr/local/bin/helloWorld

# What the container should run when it is started.
CMD ["helloWorld"]
