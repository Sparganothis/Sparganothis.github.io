use async_trait::async_trait;
use sled::IVec;
use tower_sessions::{
    cookie::time::OffsetDateTime,
    session::{Id, Record},
    session_store, ExpiredDeletion, SessionStore,
};

/// Session store backed by sled
#[derive(Debug, Clone)]
pub struct SledStore {
    /// the sled tree which should be used for storage
    sled: sled::Tree,
}

impl SledStore {
    /// Create a new SledStore using a sled tree
    ///
    /// A [`sled::Tree`] can be acquired by taking one from a [`sled::Db`]
    ///
    /// ```rust,no_run
    /// use tower_sessions_sled_store::SledStore;
    /// let sled = sled::open("storage").unwrap();
    /// let session_store = SledStore::new(sled.open_tree("session").expect("Error opening tree"));
    /// ```
    pub fn new(branch: sled::Tree) -> Self {
        Self { sled: branch }
    }
}

#[async_trait]
impl SessionStore for SledStore {
    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let encoded =
            encode_record(record).map_err(|e| session_store::Error::Encode(e.to_string()))?;

        self.sled
            .insert(record.id.0.to_be_bytes(), encoded)
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn load(&self, id: &Id) -> session_store::Result<Option<Record>> {
        let rec = self
            .sled
            .get(id.0.to_be_bytes())
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        if let Some(sr) = rec {
            let rec =
                decode_record(&sr).map_err(|e| session_store::Error::Decode(e.to_string()))?;

            return Ok(Some(rec));
        }

        Ok(None)
    }

    async fn delete(&self, id: &Id) -> session_store::Result<()> {
        self.sled
            .remove(id.0.to_be_bytes())
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }
}

/// Encode the data using rmp_serde for storage within the sled database
fn encode_record(record: &Record) -> Result<IVec, rmp_serde::encode::Error> {
    let serialized = rmp_serde::to_vec(record)?;

    Ok(IVec::from(serialized))
}

/// Decode the data using rmp_serde from the sled database
fn decode_record(data: &IVec) -> Result<Record, rmp_serde::decode::Error> {
    let decoded = rmp_serde::from_slice(data)?;

    Ok(decoded)
}

#[async_trait]
impl ExpiredDeletion for SledStore {
    /// Deletes expired sessions from the session store
    ///
    /// Note that running deletion may be expensive as this function has to iterate every session stored in the database.
    /// This may become an issue as sled technically has no idea its running under async and it may block for a long time.
    /// However to solve this this function automatically runs the deletion within [`tokio::task::spawn_blocking`]
    async fn delete_expired(&self) -> session_store::Result<()> {
        let sled = self.sled.clone();

        // deletion is ran within a sync block as sled has no concept of async and acessing the whole database may block for a long time
        tokio::task::spawn_blocking(move || -> session_store::Result<()> {
            let now = OffsetDateTime::now_utc();

            for (k, v) in sled.iter().flatten() {
                let rec =
                    decode_record(&v).map_err(|e| session_store::Error::Decode(e.to_string()))?;

                if rec.expiry_date < now {
                    sled.remove(k)
                        .map_err(|e| session_store::Error::Backend(e.to_string()))?;
                }
            }

            Ok(())
        })
        .await
        .map_err(|e| session_store::Error::Backend(e.to_string()))?
    }
}
