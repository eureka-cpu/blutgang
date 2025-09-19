//! Command line options for configuring blutgang.
//!
//! The configuration options take precedence in this order: command line, config file, defaults.
//! If a config file is present, but command line options are also present, the command line
//! options will override the config file options. If no config file is present, the default
//! configuration will be used.

#[derive(Debug, clap::Parser)]
#[command(
    name = "blutgang",
    version = crate::config::system::VERSION_STR,
    author = "makemake <vukasin@gostovic.me> and contributors",
    about = "Blutgang load balancer and cache. For more info read the wiki: https://github.com/rainshowerLabs/blutgang/wiki",
)]
pub struct Blutgang {
    /// TOML config file for blutgang.
    #[arg(
        long,
        short = 'c',
        default_value = "config.toml",
        conflicts_with = "rpc_list"
    )]
    config: std::path::PathBuf,

    /// List of RPCs in CSV format (comma separated values).
    #[arg(long, short = 'r', conflicts_with = "config")]
    rpc_list: Vec<String>,

    /// Latency moving average length.
    #[arg(long, default_value_t = 15)]
    ma_length: usize,

    /// Address to listen to.
    #[arg(long, short = 'a', default_value = "127.0.0.1")]
    address: String,

    /// Port to listen to.
    #[arg(long, short = 'p', default_value_t = 3000)]
    port: u16,

    /// Database path.
    #[arg(long, short = 'd', default_value = "blutgang-cache")]
    db_path: std::path::PathBuf,

    /// Capacity of the cache stored in memory in bytes.
    #[arg(long, default_value_t = 1000000000)]
    cache_capacity: usize,

    /// Zstd compression level.
    #[arg(long, required = false)]
    compression: i32,

    /// Time in ms to flush the DB.
    #[arg(long, default_value_t = 1000)]
    flush_every_ms: usize,

    /// Clear cache.
    #[arg(long)]
    clear: bool,

    /// Enable health checking.
    #[arg(long)]
    health_check: bool,

    /// How often to perform the health check.
    #[arg(long, default_value_t = 2000)]
    health_check_ttl: u64,

    /// Time for the RPC to respond before we remove it from the active queue.
    #[arg(long, default_value_t = 300)]
    ttl: u128,

    /// Supress the checking RPC health messages.
    #[arg(long)]
    supress_rpc_check: bool,

    /// Maximum amount of retries before we drop the current request.
    #[arg(long, default_value_t = 32)]
    max_retries: u32,

    /// Enable the admin namespace.
    #[arg(long)]
    admin: bool,

    /// Address to listen to for the admin namespace.
    #[arg(long, default_value = "127.0.0.1")]
    admin_address: String,

    /// Port to listen to for the admin namespace.
    #[arg(long, default_value_t = 5715)]
    admin_port: u16,

    /// Make the admin namespace be readonly.
    #[arg(long)]
    readonly: bool,

    /// Enable admin comms with JWT.
    #[arg(long, required = false, requires = "token")]
    jwt: bool,

    /// JWT token.
    #[arg(long, required = false, requires = "jwt")]
    token: String,
}
