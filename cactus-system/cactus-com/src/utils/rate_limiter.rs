use std::sync::Arc;
use tokio::sync::{AcquireError, OwnedSemaphorePermit, Semaphore};

pub struct RateLimiter {
    permitter: Arc<Semaphore>,
}

impl RateLimiter {
    pub fn new(limit: usize) -> Arc<Self> {
        Arc::new(Self { permitter: Arc::new(Semaphore::new(limit)) })
    }

    pub async fn get_permit(&self) -> Result<OwnedSemaphorePermit, AcquireError> {
        let permit = self.permitter.clone().acquire_owned().await?;
        Ok(permit)
    }
}