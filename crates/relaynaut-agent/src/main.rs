use anyhow::Context;
use clap::Parser;
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "relaynaut-agent")]
#[command(about = "Network probe agent for latency, jitter, and loss measurement")]
struct Args {
    /// Probe mode: udp or tcp
    #[arg(long)]
    mode: String,

    /// Target host:port
    #[arg(long)]
    target: String,

    /// Interval between probes in milliseconds
    #[arg(long, default_value = "1000")]
    interval: u64,

    /// Number of probes to send
    #[arg(long, default_value = "10")]
    count: usize,

    /// Output JSON report file
    #[arg(long)]
    json: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Initialize metrics
    metrics_exporter_prometheus::PrometheusBuilder::new()
        .install()
        .context("METRIC_INIT_ERROR")?;

    // Parse CLI arguments
    let args = Args::parse();

    tracing::info!(
        "Starting Relaynaut agent with mode: {}, target: {}",
        args.mode,
        args.target
    );

    // Placeholder for probe logic
    println!("Probes not yet implemented. Args: {:?}", args);

    Ok(())
}
