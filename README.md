# Rust API

## Installation

1. In the top-level directory run

```bash
# check your code
cargo clippy
```

2. Adjust your enviroment variables

```bash
# copy .env.example to a new file named .env
cp .env.example .env
```

```bash
# read and update the .env file with your specific configuration
nano .env
```

## Run

On the project directory

```bash
# to run the code
cargo run
```

## Appendix

- For public health checks anyone can request GET at `/api/healthcheck`
