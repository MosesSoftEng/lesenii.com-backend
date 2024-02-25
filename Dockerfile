# Use rust base image
FROM docker.io/library/rust:1.76.0-buster

# Set the working directory.
WORKDIR /app

# Copy your project files
COPY /app .

# Build your project
RUN cargo build

# Allow traffic from any IP.
ENV ROCKET_ADDRESS=0.0.0.0

# Port to use.
EXPOSE 8000

# Run your project
CMD ["cargo", "run"]
