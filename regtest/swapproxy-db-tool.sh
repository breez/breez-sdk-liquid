#!/bin/bash

# Database connection details from docker-compose.yml
DB_HOST="localhost"
DB_PORT="5433"
DB_NAME="postgres"
DB_USER="admin"
DB_PASSWORD="pass"

# Check if an option is provided
if [ $# -eq 0 ]; then
    echo "Error: No option provided"
    echo "Usage: $0 --migrate | --set-extra-fee <fee_percentage>"
    exit 1
fi

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --migrate)
            # SQL migrations
            SQL_MIGRATIONS="
            CREATE TABLE partners (
                id SERIAL PRIMARY KEY,
                email VARCHAR(255) UNIQUE NOT NULL,
                api_key VARCHAR(2047) UNIQUE NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE fee_settings (
                id SERIAL PRIMARY KEY,
                partner_id INTEGER REFERENCES partners(id),
                fee_percentage DECIMAL(5,2) NOT NULL DEFAULT 0,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(partner_id)
            );

            CREATE TABLE magic_links (
                id SERIAL PRIMARY KEY,
                partner_id INTEGER REFERENCES partners(id),
                token VARCHAR(255) NOT NULL,
                expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
                used BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE analytics (
                id SERIAL PRIMARY KEY,
                partner_id INTEGER REFERENCES partners(id),
                date DATE NOT NULL,
                total_transactions INTEGER DEFAULT 0,
                total_volume DECIMAL(20,8) DEFAULT 0,
                total_fees DECIMAL(20,8) DEFAULT 0,
                partner_revenue DECIMAL(20,8) DEFAULT 0,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );

            CREATE INDEX idx_partners_email ON partners(email);
            CREATE INDEX idx_partners_api_key ON partners(api_key);
            CREATE INDEX idx_fee_settings_partner_id ON fee_settings(partner_id);
            CREATE INDEX idx_magic_links_token ON magic_links(token);
            CREATE INDEX idx_analytics_partner_date ON analytics(partner_id, date);

            CREATE OR REPLACE FUNCTION update_timestamp()
            RETURNS TRIGGER AS \$\$
            BEGIN
                NEW.updated_at = CURRENT_TIMESTAMP;
                RETURN NEW;
            END;
            \$\$ language 'plpgsql';

            CREATE TRIGGER update_partners_timestamp
                BEFORE UPDATE ON partners
                FOR EACH ROW
                EXECUTE FUNCTION update_timestamp();

            CREATE TRIGGER update_fee_settings_timestamp
                BEFORE UPDATE ON fee_settings
                FOR EACH ROW
                EXECUTE FUNCTION update_timestamp();

            CREATE TRIGGER update_analytics_timestamp
                BEFORE UPDATE ON analytics
                FOR EACH ROW
                EXECUTE FUNCTION update_timestamp();
            "

            # Execute the migrations
            PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" << EOF
            $SQL_MIGRATIONS
EOF

            # Check if the migrations were successful
            if [ $? -eq 0 ]; then
                echo "Migrations completed successfully"
            else
                echo "Error executing migrations"
                exit 1
            fi
            shift
            ;;
        --set-extra-fee)
            if [ -z "$2" ]; then
                echo "Error: Fee percentage not provided"
                echo "Usage: $0 --set-extra-fee <fee_percentage>"
                exit 1
            fi

            # Validate fee percentage has at most 2 decimal places
            if ! [[ "$2" =~ ^[0-9]+(\.[0-9]{1,2})?$ ]]; then
                echo "Error: Fee percentage must have at most 2 decimal places"
                exit 1
            fi

            # SQL to create partner and set fee
            SQL_SET_FEE="
            INSERT INTO partners (email, api_key)
            VALUES ('extra_fee_partner@breez.technology', '')
            ON CONFLICT (email) DO NOTHING;

            INSERT INTO fee_settings (partner_id, fee_percentage)
            SELECT id, $2
            FROM partners
            WHERE email = 'extra_fee_partner@breez.technology'
            ON CONFLICT (partner_id) 
            DO UPDATE SET fee_percentage = $2;"

            # Execute the SQL
            PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" << EOF
            $SQL_SET_FEE
EOF

            # Check if the operation was successful
            if [ $? -eq 0 ]; then
                echo "Extra fee set to $2%"
            else
                echo "Error setting extra fee"
                exit 1
            fi
            shift 2
            ;;
        *)
            echo "Error: Unknown option '$1'"
            echo "Usage: $0 --migrate | --set-extra-fee <fee_percentage>"
            exit 1
            ;;
    esac
done 