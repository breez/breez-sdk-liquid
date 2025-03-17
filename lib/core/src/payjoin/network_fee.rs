pub const MIN_FEE_RATE: f64 = 0.1;

pub const WEIGHT_FIXED: usize = 222;
pub const WEIGHT_VIN_SINGLE_SIG_NATIVE: usize = 275;
pub const WEIGHT_VIN_SINGLE_SIG_NESTED: usize = 367;
pub const WEIGHT_VOUT_NESTED: usize = 270;

pub fn weight_to_vsize(weight: usize) -> usize {
    (weight + 3) / 4
}

pub fn vsize_to_fee(vsize: usize, fee_rate: f64) -> u64 {
    (vsize as f64 * fee_rate).ceil() as u64
}

pub fn weight_to_fee(weight: usize, fee_rate: f64) -> u64 {
    vsize_to_fee(weight_to_vsize(weight), fee_rate)
}

#[derive(Copy, Clone, Default)]
pub struct TxFee {
    pub server_inputs: usize,
    pub user_inputs: usize,
    pub outputs: usize,
}

impl TxFee {
    pub fn tx_weight(&self) -> usize {
        let TxFee {
            server_inputs,
            user_inputs,
            outputs,
        } = self;
        WEIGHT_FIXED
            + WEIGHT_VIN_SINGLE_SIG_NATIVE * server_inputs
            + WEIGHT_VIN_SINGLE_SIG_NESTED * user_inputs
            + WEIGHT_VOUT_NESTED * outputs
    }

    pub fn fee(&self) -> u64 {
        vsize_to_fee(weight_to_vsize(self.tx_weight()), MIN_FEE_RATE)
    }
}
