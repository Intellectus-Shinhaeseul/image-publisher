use std::{io, io::Read, path::PathBuf};
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    zenoh::init_log_from_env_or("debug");

    println!("Topic Number(1~4): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let topic_num = input.trim();
    let topic_name = String::from("camera") + topic_num;

    let mut img_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    img_path.push("image_data/image_5MB.png");

    let config_file = "zenoh.json5";
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push(config_file);
    let config = zenoh::Config::from_file(config_path).expect("Failed to load configuration file");

    println!("Opening session...");
    let session = zenoh::open(config).await.unwrap();

    println!("Declare Publisher on '{}'...", &topic_name);
    let publisher = session.declare_publisher(&topic_name).await.unwrap();

    let data = image_to_buf(&img_path);

    let mut interval = tokio::time::interval(get_duration(30));
    loop {
        interval.tick().await;
        publisher.put(&data).await.unwrap();
        // println!("Published image data to '{}'", &topic_name);
    }
}

fn get_duration(hz: i32) -> Duration {
    Duration::from_nanos((1_000_000_000f64 / hz as f64) as u64)
}

fn image_to_buf(file_path: &PathBuf) -> Vec<u8> {
    let mut file = std::fs::File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}
