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
cargo run.
```
