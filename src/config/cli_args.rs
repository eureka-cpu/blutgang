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
    db: std::path::PathBuf,

    /// Capacity of the cache stored in memory in bytes.
    #[arg(long, default_value_t = 1000000000)]
    cache_capacity: usize,

    /// Zstd compression level.
    #[arg(long)]
    compression: i32,

    /// Time in ms to flush the DB.
    #[arg(long, default_value_t = 1000)]
    flush_every_ms: usize,

    /// Clear cache.
    #[arg(long, default_value_t = false)]
    clear: bool,

    /// Enable health checking.
    #[arg(long, default_value_t = false)]
    health_check: bool,

    /// How often to perform the health check.
    #[arg(long, default_value_t = 2000)]
    health_check_ttl: u64,

    /// Time for the RPC to respond before we remove it from the active queue.
    #[arg(long, default_value_t = 300)]
    ttl: u128,

    /// Supress the checking RPC health messages.
    #[arg(long, default_value_t = true)]
    supress_rpc_check: bool,

    /// Maximum amount of retries before we drop the current request.
    #[arg(long, default_value_t = 32)]
    max_retries: u32,

    /// Enable the admin namespace.
    #[arg(long, default_value_t = true)]
    admin: bool,

    /// Address to listen to for the admin namespace.
    #[arg(long, default_value = "127.0.0.1")]
    admin_address: String,

    /// Port to listen to for the admin namespace.
    #[arg(long, default_value_t = 5715)]
    admin_port: u16,

    /// Make the admin namespace be readonly.
    #[arg(long, default_value_t = false)]
    readonly: bool,

    /// Enable admin comms with JWT.
    #[arg(long, requires = "token", default_value_t = false)]
    jwt: bool,

    /// JWT token.
    #[arg(long, requires = "jwt")]
    token: String,
}
