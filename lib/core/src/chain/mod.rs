pub(crate) mod bitcoin;
pub(crate) mod liquid;

#[macro_export]
macro_rules! get_client {
    ($chain_service:ident,$client:ident) => {
        $chain_service.set_client()?;
        let lock = $chain_service
            .client
            .read()
            .map_err(|err| anyhow!("Could not read client lock: {err:?}"))?;
        let Some($client) = lock.as_ref() else {
            bail!("Client not set"); // unreachable
        };
    };
}
