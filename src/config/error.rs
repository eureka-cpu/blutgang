//! Configuration errors

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(transparent)]
    RpcError(#[from] crate::rpc::error::RpcError),

    #[error("Node is syncing!")]
    Syncing,
}
