use std::{io, io::Read, path::PathBuf};
use time::macros::format_description;
use tokio::time::Duration;
use tracing_subscriber::fmt::time::UtcTime;

#[tokio::main]
async fn main() {
    let timer = UtcTime::new(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ));
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)  // DEBUG
        .with_timer(timer)
        .init();
    zenoh::init_log_from_env_or("debug");

    tracing::info!("Topic Number(1~4): ");
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

    tracing::info!("Opening session...");
    let session = zenoh::open(config).await.unwrap();

    tracing::info!("Declare Publisher on '{}'...", &topic_name);
    let publisher = session.declare_publisher(&topic_name).await.unwrap();

    let data = image_to_buf(&img_path);

    let mut interval = tokio::time::interval(get_duration(30));
    loop {
        interval.tick().await;
        publisher.put(&data).await.unwrap();
        tracing::trace!("Published image data to '{}'", &topic_name);
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
