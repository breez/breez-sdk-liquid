pub(crate) const MIN_FEE_RATE: f64 = 0.1;

pub(crate) const WEIGHT_FIXED: usize = 218;
pub(crate) const WEIGHT_VIN_SINGLE_SIG_NATIVE: usize = 275;
pub(crate) const WEIGHT_VIN_SINGLE_SIG_NESTED: usize = 367;
pub(crate) const WEIGHT_VOUT_NESTED: usize = 270;

#[allow(clippy::manual_div_ceil)]
pub(crate) fn weight_to_vsize(weight: usize) -> usize {
    (weight + 3) / 4
}

pub(crate) fn vsize_to_fee(vsize: usize, fee_rate: f64) -> u64 {
    (vsize as f64 * fee_rate).ceil() as u64
}

pub(crate) fn weight_to_fee(weight: usize, fee_rate: f64) -> u64 {
    vsize_to_fee(weight_to_vsize(weight), fee_rate)
}

#[derive(Copy, Clone, Default)]
pub(crate) struct TxFee {
    pub native_inputs: usize,
    pub nested_inputs: usize,
    pub outputs: usize,
}

impl TxFee {
    pub(crate) fn tx_weight(&self) -> usize {
        let TxFee {
            native_inputs,
            nested_inputs,
            outputs,
        } = self;
        WEIGHT_FIXED
            + WEIGHT_VIN_SINGLE_SIG_NATIVE * native_inputs
            + WEIGHT_VIN_SINGLE_SIG_NESTED * nested_inputs
            + WEIGHT_VOUT_NESTED * outputs
    }

    pub(crate) fn fee(&self, fee_rate: Option<f64>) -> u64 {
        weight_to_fee(self.tx_weight(), fee_rate.unwrap_or(MIN_FEE_RATE))
    }
}

#[cfg(test)]
mod tests {
    use crate::wallet::network_fee::{vsize_to_fee, weight_to_vsize};

    use super::*;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_weight_to_vsize() {
        assert_eq!(weight_to_vsize(4), 1);
        assert_eq!(weight_to_vsize(5), 2);
        assert_eq!(weight_to_vsize(7), 2);
        assert_eq!(weight_to_vsize(8), 2);
        assert_eq!(weight_to_vsize(9), 3);
        assert_eq!(weight_to_vsize(1000), 250);
    }

    #[sdk_macros::test_all]
    fn test_vsize_to_fee() {
        assert_eq!(vsize_to_fee(100, 1.0), 100);
        assert_eq!(vsize_to_fee(100, 0.5), 50);
        assert_eq!(vsize_to_fee(100, 0.1), 10);
        assert_eq!(vsize_to_fee(100, 0.11), 11);
        assert_eq!(vsize_to_fee(100, 0.15), 15);
        assert_eq!(vsize_to_fee(100, 0.151), 16);
    }

    #[sdk_macros::test_all]
    fn test_weight_to_fee() {
        assert_eq!(weight_to_fee(400, 1.0), 100);
        assert_eq!(weight_to_fee(400, 0.5), 50);
        assert_eq!(weight_to_fee(401, 1.0), 101);
        assert_eq!(weight_to_fee(399, 1.0), 100);
    }

    #[sdk_macros::test_all]
    fn test_tx_fee_calculation() {
        let fee = TxFee {
            native_inputs: 1,
            nested_inputs: 1,
            outputs: 2,
        };

        assert_eq!(
            fee.tx_weight(),
            WEIGHT_FIXED
                + WEIGHT_VIN_SINGLE_SIG_NATIVE
                + WEIGHT_VIN_SINGLE_SIG_NESTED
                + 2 * WEIGHT_VOUT_NESTED
        );

        let empty_fee = TxFee::default();
        assert_eq!(empty_fee.tx_weight(), WEIGHT_FIXED);
        assert_eq!(
            empty_fee.fee(None),
            weight_to_fee(WEIGHT_FIXED, MIN_FEE_RATE)
        );
        assert_eq!(
            empty_fee.fee(Some(MIN_FEE_RATE)),
            weight_to_fee(WEIGHT_FIXED, MIN_FEE_RATE)
        );

        let complex_fee = TxFee {
            native_inputs: 3,
            nested_inputs: 2,
            outputs: 4,
        };
        assert_eq!(
            complex_fee.tx_weight(),
            WEIGHT_FIXED
                + 3 * WEIGHT_VIN_SINGLE_SIG_NATIVE
                + 2 * WEIGHT_VIN_SINGLE_SIG_NESTED
                + 4 * WEIGHT_VOUT_NESTED
        );
    }
}
