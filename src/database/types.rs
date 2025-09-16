use std::fmt::Debug;

use sled::InlineArray;
use tokio::sync::{
    mpsc,
    oneshot,
};

/// Channel for sending requests to the database thread
///
/// The enclosing struct contains the request and a oneshot sender
/// for sending back a response.
pub type RequestBus<DB> = mpsc::UnboundedSender<DbRequest<DB>>;
pub type RequestSender<DB> = oneshot::Sender<Option<GenericDatabaseResponse<DB>>>;
// pub type RequestReceiver = oneshot::Receiver<Option<InlineArray>>;

/// A generic database layer abstraction.
pub trait GenericDatabase: Send {
    type Error: Debug;

    type ReadArgs: Debug + Send;
    type ReadReceipt: Debug + Send;

    type WriteArgs: Debug + Send;
    type WriteReceipt: Debug + Send;

    type BatchArgs: Debug + Send;
    type BatchReceipt: Debug + Send;

    type FlushArgs: Debug + Send;
    type FlushReceipt: Debug + Send;

    /// A database read operation.
    fn read(&self, args: Self::ReadArgs) -> Result<Self::ReadReceipt, Self::Error>;

    /// A database write operation.
    fn write(&self, args: Self::WriteArgs) -> Result<Self::WriteReceipt, Self::Error>;

    /// A database batch operation.
    fn batch(&self, args: Self::BatchArgs) -> Result<Self::BatchReceipt, Self::Error>;

    /// A database flush operation.
    fn flush(&self, args: Self::FlushArgs) -> Result<Self::FlushReceipt, Self::Error>;
}

impl GenericDatabase for sled::Db<{ crate::FANOUT }> {
    type Error = std::io::Error;

    type ReadArgs = Vec<u8>;
    type ReadReceipt = Option<sled::InlineArray>;

    type WriteArgs = (Vec<u8>, InlineArray);
    type WriteReceipt = Option<sled::InlineArray>;

    type BatchArgs = sled::Batch;
    type BatchReceipt = ();

    type FlushArgs = ();
    type FlushReceipt = sled::FlushStats;

    fn read(&self, key: Self::ReadArgs) -> Result<Self::ReadReceipt, Self::Error> {
        self.get(key)
    }

    fn write(&self, kv: Self::WriteArgs) -> Result<Self::WriteReceipt, Self::Error> {
        let (key, value) = kv;
        self.insert(key, value)
    }

    fn batch(&self, batch: Self::BatchArgs) -> Result<Self::BatchReceipt, Self::Error> {
        self.apply_batch(batch)
    }

    fn flush(&self, _args: Self::FlushArgs) -> Result<Self::FlushReceipt, Self::Error> {
        sled::Tree::<{ crate::FANOUT }>::flush(self)
    }
}

#[derive(Debug)]
pub enum GenericDatabaseResponse<DB: GenericDatabase> {
    Read(DB::ReadReceipt),
    Write(DB::WriteReceipt),
    Batch(DB::BatchReceipt),
    Flush(DB::FlushReceipt),
}
impl<DB: GenericDatabase> GenericDatabaseResponse<DB> {
    // TODO: @eureka-cpu -- Just a helper for now to get the inner type, but I really don't like this pattern.
    #[cfg(test)]
    pub fn into_read(self) -> DB::ReadReceipt {
        let Self::Read(read) = self else {
            panic!("database receipt was not read")
        };
        read
    }
}

/// Specifies if we are reading or writing to the DB.
#[derive(Debug)]
pub enum RequestKind<DB: GenericDatabase> {
    Read(DB::ReadArgs),
    Write(DB::WriteArgs),
    Batch(DB::BatchArgs),
    Flush(DB::FlushArgs),
}

/// Contains data to be sent to the DB thread for processing.
#[derive(Debug)]
pub struct DbRequest<DB: GenericDatabase> {
    pub request: RequestKind<DB>,
    pub sender: RequestSender<DB>,
}

impl<DB: GenericDatabase> DbRequest<DB> {
    pub fn new(request: RequestKind<DB>, sender: RequestSender<DB>) -> Self {
        DbRequest { request, sender }
    }
}
