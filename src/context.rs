use lwk_signer::SwSigner;
use lwk_wollet::{Wollet, ElectrumClient};

pub(crate) struct CliCtx {
    pub signer: SwSigner,
    pub wollet: Wollet,
    pub electrum_client: ElectrumClient,
}
