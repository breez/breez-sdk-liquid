pub(crate) fn current_migrations(is_mainnet: bool) -> Vec<&'static str> {
    let alter_payment_tx_data_add_asset_id = if is_mainnet {
        "ALTER TABLE payment_tx_data ADD COLUMN asset_id TEXT NOT NULL DEFAULT '6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d';"
    } else {
        "ALTER TABLE payment_tx_data ADD COLUMN asset_id TEXT NOT NULL DEFAULT '144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49';"
    };
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
        "
        ALTER TABLE receive_swaps ADD COLUMN pair_fees_json TEXT NOT NULL DEFAULT '';
        ALTER TABLE send_swaps ADD COLUMN pair_fees_json TEXT NOT NULL DEFAULT '';
        ALTER TABLE chain_swaps ADD COLUMN pair_fees_json TEXT NOT NULL DEFAULT '';
        ",
        "CREATE TABLE IF NOT EXISTS sync_state(
            data_id TEXT NOT NULL PRIMARY KEY,
            record_id TEXT NOT NULL,
            record_revision INTEGER NOT NULL,
            is_local INTEGER NOT NULL DEFAULT 1
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS sync_settings(
            key TEXT NOT NULL PRIMARY KEY,
            value TEXT NOT NULL
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS sync_outgoing(
            record_id TEXT NOT NULL PRIMARY KEY,
            data_id TEXT NOT NULL UNIQUE,
            record_type INTEGER NOT NULL,
            commit_time INTEGER NOT NULL,
            updated_fields_json TEXT
        ) STRICT;",
        "CREATE TABLE IF NOT EXISTS sync_incoming(
            record_id TEXT NOT NULL PRIMARY KEY,
            revision INTEGER NOT NULL UNIQUE,
            schema_version TEXT NOT NULL,
            data BLOB NOT NULL
        ) STRICT;",
        "ALTER TABLE receive_swaps DROP COLUMN mrh_script_pubkey;",
        "ALTER TABLE payment_details ADD COLUMN lnurl_info_json TEXT;",
        "ALTER TABLE payment_tx_data ADD COLUMN unblinding_data TEXT;",
        "ALTER TABLE chain_swaps ADD COLUMN actual_payer_amount_sat INTEGER;",
        "ALTER TABLE chain_swaps ADD COLUMN accepted_receiver_amount_sat INTEGER;",
        "
        ALTER TABLE receive_swaps ADD COLUMN timeout_block_height INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE send_swaps ADD COLUMN timeout_block_height INTEGER NOT NULL DEFAULT 0;
        ",
        "
        ALTER TABLE receive_swaps ADD COLUMN version INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE send_swaps ADD COLUMN version INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE chain_swaps ADD COLUMN version INTEGER NOT NULL DEFAULT 0;
        CREATE TRIGGER IF NOT EXISTS update_receive_swaps_version
        AFTER UPDATE ON receive_swaps
        BEGIN
            UPDATE receive_swaps SET version = version + 1
            WHERE id = NEW.id;
        END;
        CREATE TRIGGER IF NOT EXISTS update_send_swaps_version
        AFTER UPDATE ON send_swaps
        BEGIN
            UPDATE send_swaps SET version = version + 1
            WHERE id = NEW.id;
        END;
        CREATE TRIGGER IF NOT EXISTS update_chain_swaps_version
        AFTER UPDATE ON chain_swaps
        BEGIN
            UPDATE chain_swaps SET version = version + 1
            WHERE id = NEW.id;
        END;
        ",
        "
        ALTER TABLE receive_swaps ADD COLUMN destination_pubkey TEXT;
        ALTER TABLE send_swaps ADD COLUMN destination_pubkey TEXT;
        ",
        "ALTER TABLE chain_swaps ADD COLUMN auto_accepted_fees INTEGER NOT NULL DEFAULT 0;",
        alter_payment_tx_data_add_asset_id,
        "
        ALTER TABLE payment_tx_data RENAME COLUMN amount_sat TO amount;
        UPDATE payment_tx_data SET amount = amount - fees_sat WHERE payment_type = 1;
        ",
    ]
}
