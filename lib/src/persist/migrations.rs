pub(crate) fn current_migrations() -> Vec<&'static str> {
    vec![
        "CREATE TABLE IF NOT EXISTS ongoing_swaps (
            id TEXT NOT NULL PRIMARY KEY,
            preimage TEXT NOT NULL,
            redeem_script TEXT NOT NULL,
            blinding_key TEXT NOT NULL
        ) STRICT;",
    ]
}
