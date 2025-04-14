use breez_sdk_liquid::wallet::persister::lwk_wollet::Update;

// If both updates are only tip updates, we can merge them.
// See https://github.com/Blockstream/lwk/blob/0322a63310f8c8414c537adff68dcbbc7ff4662d/lwk_wollet/src/persister.rs#L174
pub(crate) fn maybe_merge_updates(
    mut new_update: Update,
    prev_update: Option<&Update>,
    mut next_index: usize,
) -> (Update, /*index*/ usize) {
    if new_update.only_tip() {
        if let Some(prev_update) = prev_update {
            if prev_update.only_tip() {
                new_update.wollet_status = prev_update.wollet_status;
                next_index -= 1;
            }
        }
    }
    (new_update, next_index)
}

#[cfg(any(feature = "browser", feature = "node-js"))]
#[cfg(test)]
pub(crate) mod tests {
    use crate::platform::create_wallet_persister;
    use breez_sdk_liquid::elements::hashes::Hash;
    use breez_sdk_liquid::elements::{BlockHash, BlockHeader, TxMerkleNode, Txid};
    use breez_sdk_liquid::model::{LiquidNetwork, Signer};
    use breez_sdk_liquid::signer::{SdkLwkSigner, SdkSigner};
    use breez_sdk_liquid::wallet::get_descriptor;
    use breez_sdk_liquid::wallet::persister::lwk_wollet::WolletDescriptor;
    use breez_sdk_liquid::wallet::persister::{lwk_wollet, WalletCachePersister};
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio_with_wasm::alias as tokio;

    #[cfg(feature = "browser")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    pub(crate) fn get_wollet_descriptor() -> anyhow::Result<WolletDescriptor> {
        let signer: Rc<Box<dyn Signer>> = Rc::new(Box::new(SdkSigner::new("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about", "", false)?));
        let sdk_lwk_signer = SdkLwkSigner::new(signer)?;
        Ok(get_descriptor(&sdk_lwk_signer, LiquidNetwork::Testnet)?)
    }

    async fn build_persister(working_dir: &str) -> anyhow::Result<Rc<dyn WalletCachePersister>> {
        let desc = get_wollet_descriptor()?;
        create_wallet_persister(
            &PathBuf::from(working_dir),
            desc,
            LiquidNetwork::Testnet,
            "aaaaaaaa",
        )
        .await
    }

    #[sdk_macros::async_test_wasm]
    async fn test_wallet_cache() -> anyhow::Result<()> {
        let working_dir = format!("/tmp/{}", uuid::Uuid::new_v4());

        let persister = build_persister(&working_dir).await?;
        let lwk_persister = persister.get_lwk_persister()?;

        assert!(lwk_persister.get(0)?.is_none());

        lwk_persister.push(get_lwk_update(5, false))?;

        assert_eq!(lwk_persister.get(0)?.unwrap().tip.height, 5);
        assert!(lwk_persister.get(1)?.is_none());

        lwk_persister.push(get_lwk_update(10, true))?;

        assert_eq!(lwk_persister.get(0)?.unwrap().tip.height, 5);
        assert_eq!(lwk_persister.get(1)?.unwrap().tip.height, 10);
        assert!(lwk_persister.get(2)?.is_none());

        lwk_persister.push(get_lwk_update(15, true))?;

        assert_eq!(lwk_persister.get(0)?.unwrap().tip.height, 5);
        assert_eq!(lwk_persister.get(1)?.unwrap().tip.height, 15);
        assert!(lwk_persister.get(2)?.is_none());

        // Allow persister task to persist updates when persister is async
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Reload persister
        let persister = build_persister(&working_dir).await?;
        let lwk_persister = persister.get_lwk_persister()?;

        assert_eq!(lwk_persister.get(0)?.unwrap().tip.height, 5);
        assert_eq!(lwk_persister.get(1)?.unwrap().tip.height, 15);
        assert!(lwk_persister.get(2)?.is_none());

        persister.clear_cache().await?;
        assert!(lwk_persister.get(0)?.is_none());
        assert!(lwk_persister.get(1)?.is_none());
        assert!(lwk_persister.get(2)?.is_none());

        lwk_persister.push(get_lwk_update(20, false))?;
        assert_eq!(lwk_persister.get(0)?.unwrap().tip.height, 20);
        assert!(lwk_persister.get(1)?.is_none());

        Ok(())
    }

    pub(crate) fn get_lwk_update(height: u32, only_tip: bool) -> lwk_wollet::Update {
        let txid_height_new = match only_tip {
            true => Vec::new(),
            false => {
                vec![(Txid::all_zeros(), None)]
            }
        };
        lwk_wollet::Update {
            version: 1,
            wollet_status: 0,
            new_txs: Default::default(),
            txid_height_new,
            txid_height_delete: vec![],
            timestamps: vec![],
            scripts_with_blinding_pubkey: vec![],
            tip: BlockHeader {
                version: 0,
                prev_blockhash: BlockHash::all_zeros(),
                merkle_root: TxMerkleNode::all_zeros(),
                time: 0,
                height,
                ext: Default::default(),
            },
        }
    }
}
