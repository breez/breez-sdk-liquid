pub(crate) struct OngoingSwap {
    pub id: String,
    pub preimage: String,
    pub redeem_script: String,
    pub blinding_key: String,
    pub requested_amount_sat: u64,
}