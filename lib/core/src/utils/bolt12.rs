use anyhow::{anyhow, ensure, Result};
use sdk_common::bitcoin::bech32::ToBase32;
use sdk_common::bitcoin::bech32::{self, FromBase32};
use sdk_common::lightning_with_bolt12::offers::invoice::Bolt12Invoice;
use sdk_common::lightning_with_bolt12::offers::invoice_request::InvoiceRequest;
use sdk_common::lightning_with_bolt12::offers::offer::Offer;
use sdk_common::lightning_with_bolt12::util::ser::Writeable;

pub fn encode_invoice(invoice: &Bolt12Invoice) -> Result<String> {
    let mut writer = Vec::new();
    invoice.write(&mut writer)?;

    Ok(bech32::encode_without_checksum("lni", writer.to_base32())?)
}

pub fn encode_offer(offer: &Offer) -> Result<String> {
    let mut writer = Vec::new();
    offer.write(&mut writer)?;

    Ok(bech32::encode_without_checksum("lno", writer.to_base32())?)
}

/// Parsing logic that decodes a string into a [Bolt12Invoice].
///
/// It matches the encoding logic on Boltz side.
pub(crate) fn decode_invoice(invoice: &str) -> Result<Bolt12Invoice> {
    let (hrp, data) = bech32::decode_without_checksum(invoice)?;
    ensure!(hrp.as_str() == "lni", "Invalid HRP");

    let data = Vec::<u8>::from_base32(&data)?;

    sdk_common::lightning_with_bolt12::offers::invoice::Bolt12Invoice::try_from(data)
        .map_err(|e| anyhow!("Failed to parse BOLT12: {e:?}"))
}

pub(crate) fn decode_invoice_request(invoice_request: &str) -> Result<InvoiceRequest> {
    InvoiceRequest::try_from(
        hex::decode(invoice_request).map_err(|e| anyhow!("Cannot decode invoice request: {e}"))?,
    )
    .map_err(|e| anyhow!("Cannot parse invoice request: {e:?}"))
}
