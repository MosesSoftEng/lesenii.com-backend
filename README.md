# lesenii.com-backend

Backend for lesenii.com using rust, rocket and diesel

# Installations.

## Rocket Web Framework.

```bash
# Install C compiler.
sudo apt update
sudo apt install -y gcc

# Install latest toolchain.
rustup default stable


# Create a new binary-based Cargo project.
cargo new app --bin


# Add Rocket as a dependency in Cargo.toml.
[dependencies]
rocket = "=0.5.0-rc.4"


# Add missing dependecies.
cargo add futures-util
cargo add lib
cargo add mio

```

# Run application.

```bash
cargo clean

# Create debug build.
cargo build

# Run debug build.
cargo run.

# Create release build.
cargo build --release

# Run release build.
cargo run --release
```

# Containerization.
```bash
# Pull rust image.
podman pull rust

# Create docker image from current directory.
podman build --tag lesenii:dev1.0.1 .

# List docker images.
podman images

# Run docker image with tag latest.
podman run  -p 8000:8000 localhost/lesenii:dev1.0.

# Enter running docker container.
podman exec -it <CONTAINER_ID> /bin/bash 
```

# Continous Integration.
```bash
# Create a .github/workflows directory.
mkdir -p .github/workflows


touch .github/workflows/backend-ci-cd.yml

```