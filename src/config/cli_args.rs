//! Command line options for configuring blutgang.
//!
//! The configuration options take precedence in this order: command line, config file, defaults.
//! If a config file is present, but command line options are also present, the command line
//! options will override the config file options. If no config file is present, the default
//! configuration will be used.

use clap::builder::styling;

/// The terminal output style configuration.
pub const TERM_STYLE: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Cyan.on_default())
    .placeholder(styling::AnsiColor::Cyan.on_default())
    .valid(styling::AnsiColor::Cyan.on_default());

// Help Headers
const CORE_OPTS: &str = "Core Configuration Options";
const CACHE_OPTS: &str = "Cache Options";
const ADMIN_OPTS: &str = "Admin Namespace Options";

// TODO: @eureka-cpu -- Add environment variables
#[derive(Debug, clap::Parser)]
#[command(
    name = "blutgang",
    version = crate::config::system::VERSION_STR,
    author,
    about = "Blutgang load balancer and cache. For more info read the wiki: https://github.com/rainshowerLabs/blutgang/wiki",
)]
pub struct Blutgang {
    // -- Core Configuration Options
    //
    /// Path to a TOML config file for blutgang.
    #[arg(long, short = 'c', default_value = "config.toml", help_heading = CORE_OPTS)]
    config: std::path::PathBuf,

    /// Specify RPC endpoints.
    #[arg(long = "rpc", short = 'r', help_heading = CORE_OPTS)]
    rpc_list: Vec<String>,

    /// Address to listen to.
    #[arg(long, short = 'a', help_heading = CORE_OPTS)]
    address: Option<String>,

    /// Port to listen to.
    #[arg(long, short = 'p', help_heading = CORE_OPTS)]
    port: Option<u16>,

    /// Latency moving average length.
    #[arg(long, help_heading = CORE_OPTS)]
    ma_length: Option<usize>,

    /// Time for the RPC to respond before we remove it from the active queue.
    #[arg(long, help_heading = CORE_OPTS)]
    ttl: Option<u128>,

    /// Maximum amount of retries before we drop the current request.
    #[arg(long, help_heading = CORE_OPTS)]
    max_retries: Option<u32>,

    /// Block time in ms.
    #[arg(long, help_heading = CORE_OPTS)]
    expected_block_time: Option<u64>,

    /// How often to perform the health check.
    #[arg(long, help_heading = CORE_OPTS)]
    health_check_ttl: Option<u64>,

    /// Clear cache.
    #[arg(long, help_heading = CORE_OPTS)]
    clear_cache: bool,
    #[arg(long, hide = true, conflicts_with = "clear_cache")]
    no_clear_cache: bool,

    /// Sort RPCs by latency on startup.
    #[arg(long, help_heading = CORE_OPTS)]
    sort_on_startup: bool,
    #[arg(long, hide = true, conflicts_with = "sort_on_startup")]
    no_sort_on_startup: bool,

    /// Enable health checking.
    #[arg(long, help_heading = CORE_OPTS)]
    health_check: bool,
    #[arg(long, hide = true, conflicts_with = "health_check")]
    no_health_check: bool,

    /// Enable content type header checking. Useful if you want
    /// Blutgang to be JSON-RPC compliant.
    #[arg(long, help_heading = CORE_OPTS)]
    header_check: bool,
    #[arg(long, hide = true, conflicts_with = "header_check")]
    no_header_check: bool,

    /// Supress the checking RPC health messages.
    #[arg(long, help_heading = CORE_OPTS)]
    supress_rpc_check: bool,
    #[arg(long, hide = true, conflicts_with = "supress_rpc_check")]
    no_supress_rpc_check: bool,

    // -- Cache Options
    //
    /// Database path.
    #[arg(long, short = 'd', help_heading = CACHE_OPTS)]
    db_path: Option<std::path::PathBuf>,

    /// Enable the sled database backend.
    #[arg(long, conflicts_with = "rocksdb", help_heading = CACHE_OPTS)]
    sled: bool,

    /// Enable the rocksdb database backend.
    #[arg(long, conflicts_with = "sled", help_heading = CACHE_OPTS)]
    rocksdb: bool,

    // -- Admin Namespace Options
    //
    /// Path to a privileged admin config.
    #[arg(long, help_heading = ADMIN_OPTS)]
    admin_path: Option<std::path::PathBuf>,

    /// Enable the admin namespace.
    #[arg(long, help_heading = ADMIN_OPTS)]
    admin: bool,
    #[arg(long, hide = true, conflicts_with = "admin")]
    no_admin: bool,

    /// Address to listen to for the admin namespace.
    #[arg(long, help_heading = ADMIN_OPTS)]
    admin_address: Option<String>,

    /// Port to listen to for the admin namespace.
    #[arg(long, help_heading = ADMIN_OPTS)]
    admin_port: Option<u16>,

    /// Make the admin namespace readonly.
    #[arg(long, help_heading = ADMIN_OPTS)]
    admin_readonly: bool,
    #[arg(long, hide = true, conflicts_with = "admin_readonly")]
    no_admin_readonly: bool,

    /// Enable admin comms with JWT.
    #[arg(long, required = false, requires = "admin_key", help_heading = ADMIN_OPTS)]
    admin_jwt: bool,
    #[arg(long, hide = true, conflicts_with_all = ["admin_key", "admin_jwt"])]
    no_admin_jwt: bool,

    /// JWT token.
    #[arg(long, requires = "admin_jwt", help_heading = ADMIN_OPTS)]
    admin_key: Option<String>,
}
