use crate::{
    database::types::{
        DbRequest,
        GenericDatabase,
        GenericDatabaseResponse,
        RequestKind,
    },
    log_err,
};

use tokio::sync::mpsc;

/// Processes incoming requests from clients and returns responses
pub async fn database_processing<DB: GenericDatabase + std::fmt::Debug>(
    mut rax: mpsc::UnboundedReceiver<DbRequest<DB>>,
    cache: DB,
) {
    loop {
        while let Some(incoming) = rax.recv().await {
            let result = match incoming.request {
                RequestKind::Read(k) => cache.read(k).map(GenericDatabaseResponse::Read),
                RequestKind::Write(kv) => cache.write(kv).map(GenericDatabaseResponse::Write),
                RequestKind::Batch(b) => cache.batch(b).map(GenericDatabaseResponse::Batch),
                RequestKind::Flush(a) => cache.flush(a).map(GenericDatabaseResponse::Flush),
            };

            if result.is_err() {
                log_err!("Db failed to send response back: {:?}", result);
                let _ = incoming.sender.send(None);
                continue;
            }

            let _ = incoming.sender.send(result.ok());
        }
    }
}

/// Macro to abstract getting the data from the DB.
///
/// Returns `Option<InlineArray>`, where the result is `None` if
/// there was an error or data isn't present, or `Some` if the operation
/// completed successfully.
#[macro_export]
macro_rules! db_get {
    (
        $channel:expr,
        $data:expr
    ) => {{
        use $crate::database::types::{
            DbRequest,
            RequestKind,
        };

        let (tx, rx) = tokio::sync::oneshot::channel();
        let req = DbRequest::new(RequestKind::Read($data), tx);

        let _ = $channel.send(req);

        rx.await
    }};
}

/// Macro to abstract inserting data into the DB.
///
/// Returns `Option<InlineArray>`, where the result is `None` if
/// there was an error or data isn't present, or `Some` if the operation
/// completed successfully.
#[macro_export]
macro_rules! db_insert {
    (
        $channel:expr,
        $k:expr,
        $v:expr
    ) => {{
        let (tx, rx) = oneshot::channel();
        let req = DbRequest::new(RequestKind::Write(($k, $v)), tx);

        let _ = $channel.send(req);

        rx
    }};
}

/// Macro to abstract writing batch data to the DB.
///
/// Returns `Option<InlineArray>`, where the result is `None` if
/// there was an error or data isn't present, or `Some` if the operation
/// completed successfully.
#[macro_export]
macro_rules! db_batch {
    (
        $channel:expr,
        $data:expr
    ) => {{
        let (tx, rx) = oneshot::channel();
        let req = DbRequest::new(RequestKind::Batch($data), tx);

        let _ = $channel.send(req);

        rx
    }};
}

/// Macro for flushing the DB
///
/// Returns `Option<InlineArray>`, where the result is `None` if
/// there was an error or data isn't present, or `Some` if the operation
/// completed successfully.
#[macro_export]
macro_rules! db_flush {
    (
        $channel:expr,
        $data:expr
    ) => {{
        use $crate::database::types::{
            DbRequest,
            RequestKind,
        };

        use tokio::sync::oneshot;

        let (tx, rx) = oneshot::channel();
        let req = DbRequest::new(RequestKind::Flush($data), tx);

        let _ = $channel.send(req);

        rx
    }};
}
