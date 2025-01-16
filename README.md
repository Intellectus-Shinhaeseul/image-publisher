# Image Publisher Package

This Rust package is designed to publish **5MB images at 30Hz** using `zenoh`.

## Features
- Publishes high-resolution images at a consistent rate of 30Hz.
- Dynamically assigns a topic name based on user input.

## How It Works
1. Run the package using `cargo run -- $camera_num`.
2. You will be prompted to enter a number:
   - The number you enter will determine the topic name.
   - **Example:**
     - If you enter `1`, the topic name will be `camera1`.
     - If you enter `2`, the topic name will be `camera2`.
3. The package will begin publishing images to the specified topic using `zenoh`.

## Example Usage
```bash
$ cargo run -- 1
Publishing images on topic: camera1
```

## Dependencies
This project uses `zenoh` as its core communication middleware. Ensure you have it included in your `Cargo.toml` file:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
zenoh = { version = "1.1.0" }
```

### Setting up the Zenoh Router
This publisher operates in **peer mode**, publishing through a Zenoh router. Use the provided `router.json5` file to configure and run the Zenoh router:

```bash
zenohd -c router.json5
```

### Clone and Build the Project

Clone this repository:

```bash
git clone https://github.com/Intellectus-Shinhaeseul/image-publisher.git
cd image-publisher
```

Build the project:

```bash
cargo build --release
```

Run the project:

```bash
cargo run
