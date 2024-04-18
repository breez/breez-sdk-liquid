pub(crate) fn current_migrations() -> Vec<&'static str> {
    vec![
        "CREATE TABLE IF NOT EXISTS ongoing_receive_swaps (
            id TEXT NOT NULL PRIMARY KEY,
            preimage TEXT NOT NULL,
            redeem_script TEXT NOT NULL,
            blinding_key TEXT NOT NULL,
            invoice TEXT NOT NULL,
            receiver_amount_sat INTEGER NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS ongoing_send_swaps (
            id TEXT NOT NULL PRIMARY KEY,
            funding_address TEXT NOT NULL,
            invoice TEXT NOT NULL,
            receiver_amount_sat INTEGER NOT NULL,
            txid TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS payment_data(
            id TEXT NOT NULL PRIMARY KEY,
            payer_amount_sat INTEGER NOT NULL
        ) STRICT;",
    ]
}
