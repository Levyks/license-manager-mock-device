use tokio::time;

mod config;
mod geo;
mod api;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <.yml config file path>", args[0]);
        std::process::exit(1);
    }

    let config = config::read_config(&args[1]).await.unwrap();

    println!("{:?}", config);

    let mut interval = time::interval(time::Duration::from_secs(config.interval));

    loop {
        interval.tick().await;

        for device in &config.devices {
            let current_position = geo::calculate_current_position(&device.start, &device.end, config.speed);
            println!("License = {}, {:?}", device.license_id, current_position);
            api::send_heartbeat(&config.base_url, &device.license_id, &current_position).await.unwrap_or_else(|e| {
                println!("Error sending heartbeat: {:?}", e);
            });
        }

    }
}
