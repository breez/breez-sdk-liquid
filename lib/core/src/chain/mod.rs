pub(crate) mod bitcoin;
pub(crate) mod liquid;

use std::future::Future;
use std::time::Duration;

use anyhow::Result;
use log::info;
use tokio_with_wasm::alias as tokio;

async fn with_empty_retry<F, Fut, T>(mut f: F, retries: u64) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
    T: AsRef<[<T as IntoIterator>::Item]> + IntoIterator,
{
    let mut retry = 0;
    loop {
        match f().await {
            Ok(res) => {
                if res.as_ref().is_empty() {
                    if retry == retries {
                        return Ok(res);
                    }
                    retry += 1;
                    info!("Empty result, retrying in 1 second...");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                } else {
                    return Ok(res);
                }
            }
            Err(e) => return Err(e),
        }
    }
}

async fn with_error_retry<F, Fut, T>(mut f: F, retries: u64) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut retry = 0;
    loop {
        match f().await {
            Ok(res) => return Ok(res),
            Err(e) => {
                if retry == retries {
                    return Err(e);
                }
                retry += 1;
                info!("Error occurred: {e}, retrying in {retry} seconds...");
                tokio::time::sleep(Duration::from_secs(retry)).await;
            }
        }
    }
}
