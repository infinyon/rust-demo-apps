//! Consume Crate
//!
//! # Consumer Example
//!
//! 1. Consumer connects to a fluvio cluster
//! 2. Reads data streams with the following parameters
//!     * topic:      **my-topic-1**
//!     * partition:  **0**
//!     * offset:     **FetchOffset::Earliest** (without an offset)
//!
//! ```
//! let config = ScConfig::new(None, None).expect("connect");
//! let mut client = config.connect().await.expect("should connect");
//!
//! let mut replica = client
//!     .find_replica_for_topic_partition("my-topic-1", 0)
//!     .await
//!     .expect("replica missing");
//! let mut log_stream = replica.fetch_logs(FetchOffset::Earliest(None), FetchLogOption::default());
//!
//! while let Some(response) = log_stream.next().await {
//!     let records = response.records;
//!     for batch in records.batches {
//!         for record in batch.records {
//!             if let Some(bytes) = record.value().inner_value() {
//!                 let msg = String::from_utf8(bytes).expect("string");
//!                 println!("{}", msg);
//!             }
//!         }
//!     }
//! }
//! ```
use futures::stream::StreamExt;

use flv_client::profile::ScConfig;
use flv_client::ClientError;
use flv_client::FetchLogOption;
use flv_client::FetchOffset;
use flv_client::ReplicaLeader;
use flv_client::SpuController;
use flv_future_aio::task::run_block_on;

fn main() {
    run_block_on(consume()).expect("run");
}

/// Adds one to the number given.
async fn consume() -> Result<(), ClientError> {
    let config = ScConfig::new(None, None).expect("connect");
    let mut client = config.connect().await.expect("should connect");

    // look-up stream for "my-topic-1"
    let topic = "my-topic-1";
    let partition = 0;
    let offset = FetchOffset::Earliest(None);

    let mut replica = client
        .find_replica_for_topic_partition(topic, partition)
        .await
        .expect("replica missing");
    let mut log_stream = replica.fetch_logs(offset, FetchLogOption::default());

    // read read from producer and print to terminal
    while let Some(response) = log_stream.next().await {
        let records = response.records;
        for batch in records.batches {
            for record in batch.records {
                if let Some(bytes) = record.value().inner_value() {
                    let msg = String::from_utf8(bytes).expect("string");
                    println!("{}", msg);
                }
            }
        }
    }

    Ok(())
}
