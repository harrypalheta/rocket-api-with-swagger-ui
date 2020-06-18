# Rocket API with Swagger UI

This is a simple Rest API containing "Hello, World!" made in the Rocket Framework for the Rust language and with Swagger UI.

Recommended for novice users in Rust.

For more information access:

* https://rocket.rs/
* https://www.rust-lang.org/
* https://swagger.io/tools/swagger-ui/

> This repository was inspired by:
> * https://github.com/GREsau/okapi

## How to Use

> Procedure tested on **Ubuntu 20.04**

1. Install Rust Lang:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Configure the environment variables:

```sh
echo 'source $HOME/.cargo/env' >> ~/.bashrc
```

3. Load the environment variables:

```sh
source ~/.bashrc
```

4. Enable unstable language features:

```sh
rustup default nightly
```

5. Run the project:

```sh
cargo run
```

6. Access the links in the browser:

* http://localhost:8000/
* http://localhost:8000/swagger-ui/index.html