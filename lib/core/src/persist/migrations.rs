pub(crate) fn current_migrations() -> Vec<&'static str> {
    vec![
        "CREATE TABLE IF NOT EXISTS receive_swaps (
            id TEXT NOT NULL PRIMARY KEY,
            preimage TEXT NOT NULL,
            create_response_json TEXT NOT NULL,
            blinding_key TEXT NOT NULL,
            invoice TEXT NOT NULL,
            payer_amount_sat INTEGER NOT NULL,
            receiver_amount_sat INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            claim_fees_sat INTEGER NOT NULL,
            claim_tx_id TEXT
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS send_swaps (
           id TEXT NOT NULL PRIMARY KEY,
           invoice TEXT NOT NULL,
           payer_amount_sat INTEGER NOT NULL,
           receiver_amount_sat INTEGER NOT NULL,
           create_response_json TEXT NOT NULL,
           lockup_tx_id TEXT,
           is_claim_tx_seen INTEGER NOT NULL DEFAULT 0,
           created_at INTEGER NOT NULL
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS payment_tx_data (
            tx_id TEXT NOT NULL PRIMARY KEY,
            payment_type INTEGER NOT NULL,
            status INTEGER NOT NULL,
            timestamp INTEGER,
            amount_sat INTEGER NOT NULL
        ) STRICT;",
    ]
}
