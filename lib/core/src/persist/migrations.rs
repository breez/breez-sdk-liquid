pub(crate) fn current_migrations() -> Vec<&'static str> {
    vec![
        "CREATE TABLE IF NOT EXISTS receive_swaps (
            id TEXT NOT NULL PRIMARY KEY,
            preimage TEXT NOT NULL,
            create_response_json TEXT NOT NULL,
            claim_private_key TEXT NOT NULL,
            invoice TEXT NOT NULL,
            payer_amount_sat INTEGER NOT NULL,
            receiver_amount_sat INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            claim_fees_sat INTEGER NOT NULL,
            claim_tx_id TEXT,
            lockup_tx_id TEXT,
            state INTEGER NOT NULL
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS send_swaps (
            id TEXT NOT NULL PRIMARY KEY,
            invoice TEXT NOT NULL UNIQUE,
            preimage TEXT,
            payer_amount_sat INTEGER NOT NULL,
            receiver_amount_sat INTEGER NOT NULL,
            create_response_json TEXT NOT NULL,
            refund_private_key TEXT NOT NULL,
            lockup_tx_id TEXT,
            refund_tx_id TEXT,
            created_at INTEGER NOT NULL,
            state INTEGER NOT NULL
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS payment_tx_data (
            tx_id TEXT NOT NULL PRIMARY KEY,
            payment_type INTEGER NOT NULL,
            is_confirmed INTEGER NOT NULL,
            timestamp INTEGER,
            amount_sat INTEGER NOT NULL,
            fees_sat INTEGER NOT NULL
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS chain_swaps (
            id TEXT NOT NULL PRIMARY KEY,
            direction INTEGER NOT NULL,
            claim_address TEXT NOT NULL,
            lockup_address TEXT NOT NULL,
            timeout_block_height INTEGER NOT NULL,
            preimage TEXT NOT NULL,
            payer_amount_sat INTEGER NOT NULL,
            receiver_amount_sat INTEGER NOT NULL,
            accept_zero_conf INTEGER NOT NULL,
            create_response_json TEXT NOT NULL,
            claim_private_key TEXT NOT NULL,
            refund_private_key TEXT NOT NULL,
            server_lockup_tx_id TEXT,
            user_lockup_tx_id TEXT,
            claim_fees_sat INTEGER NOT NULL,
            claim_tx_id TEXT,
            refund_tx_id TEXT,
            created_at INTEGER NOT NULL,
            state INTEGER NOT NULL
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS cached_items (
            key TEXT NOT NULL PRIMARY KEY,
            value TEXT NOT NULL
        ) STRICT;",
        "
        ALTER TABLE receive_swaps ADD COLUMN description TEXT;
        ALTER TABLE send_swaps ADD COLUMN description TEXT;
        ALTER TABLE chain_swaps ADD COLUMN description TEXT;
        ",
        "CREATE TABLE IF NOT EXISTS payment_details (
            tx_id TEXT NOT NULL PRIMARY KEY,
            destination TEXT NOT NULL,
            description TEXT NOT NULL
        );",
        "
        ALTER TABLE receive_swaps ADD COLUMN id_hash TEXT;
        ALTER TABLE send_swaps ADD COLUMN id_hash TEXT;
        ALTER TABLE chain_swaps ADD COLUMN id_hash TEXT;
        ",
        "
        ALTER TABLE payment_details RENAME TO payment_details_old;

        CREATE TABLE IF NOT EXISTS payment_details (
            tx_id TEXT NOT NULL PRIMARY KEY,
            destination TEXT NOT NULL,
            description TEXT
        ) STRICT;
        
        INSERT INTO payment_details
         (tx_id, destination, description)
         SELECT 
            tx_id,
            destination,
            description
         FROM payment_details_old;
        
        DROP TABLE payment_details_old;            
        ",
        "
        ALTER TABLE receive_swaps ADD COLUMN payment_hash TEXT;
        ALTER TABLE send_swaps ADD COLUMN payment_hash TEXT;
        ",
        "
        CREATE TABLE IF NOT EXISTS reserved_addresses (
            address TEXT NOT NULL PRIMARY KEY,
            expiry_block_height INTEGER NOT NULL
        ) STRICT;

        ALTER TABLE receive_swaps ADD COLUMN mrh_address TEXT NOT NULL DEFAULT '';
        ALTER TABLE receive_swaps ADD COLUMN mrh_script_pubkey TEXT NOT NULL DEFAULT '';
        ALTER TABLE receive_swaps ADD COLUMN mrh_tx_id TEXT;
        ",
        "
        ALTER TABLE chain_swaps RENAME TO old_chain_swaps;

        CREATE TABLE IF NOT EXISTS chain_swaps (
            id TEXT NOT NULL PRIMARY KEY,
            direction INTEGER NOT NULL,
            claim_address TEXT,
            lockup_address TEXT NOT NULL,
            timeout_block_height INTEGER NOT NULL,
            preimage TEXT NOT NULL,
            payer_amount_sat INTEGER NOT NULL,
            receiver_amount_sat INTEGER NOT NULL,
            accept_zero_conf INTEGER NOT NULL,
            create_response_json TEXT NOT NULL,
            claim_private_key TEXT NOT NULL,
            refund_private_key TEXT NOT NULL,
            server_lockup_tx_id TEXT,
            user_lockup_tx_id TEXT,
            claim_fees_sat INTEGER NOT NULL,
            claim_tx_id TEXT,
            refund_tx_id TEXT,
            created_at INTEGER NOT NULL,
            state INTEGER NOT NULL,
            description TEXT,
            id_hash TEXT
        ) STRICT;

        INSERT INTO chain_swaps (
            id, 
            direction,
            claim_address,
            lockup_address,
            timeout_block_height,
            preimage,
            payer_amount_sat,
            receiver_amount_sat,
            accept_zero_conf,
            create_response_json,
            claim_private_key,
            refund_private_key,
            server_lockup_tx_id,
            user_lockup_tx_id,
            claim_fees_sat,
            claim_tx_id,
            refund_tx_id,
            created_at,
            state,
            description,
            id_hash
        ) SELECT 
            id, 
            direction,
            claim_address,
            lockup_address,
            timeout_block_height,
            preimage,
            payer_amount_sat,
            receiver_amount_sat,
            accept_zero_conf,
            create_response_json,
            claim_private_key,
            refund_private_key,
            server_lockup_tx_id,
            user_lockup_tx_id,
            claim_fees_sat,
            claim_tx_id,
            refund_tx_id,
            created_at,
            state,
            description,
            id_hash
        FROM old_chain_swaps;

        DROP TABLE old_chain_swaps;
        ",
        "ALTER TABLE send_swaps ADD COLUMN bolt12_offer TEXT;",
    ]
}
