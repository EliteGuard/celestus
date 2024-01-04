# Initial setup

vault login - login with root token
vault token create -id test-dev

vault login - login with test-dev
vault auth enable approle - only once

# Add policy

path "kv/data/dev/celestus/*" {
capabilities = [ "read" ]
}

# k/v secrets

dev/celestus/database/pg

{
"PG_DATABASE_NAME": "celestus",
"PG_HOST": "localhost",
"PG_PASSWORD": "123456",
"PG_PORT": "5433",
"PG_URL": "postgres://dev:123456@localhost:5433/celestus",
"PG_URL_PREFIX": "postgres",
"PG_USER": "dev"
}

# Add approle with policy

vault write auth/approle/role/celestus token_num_uses=0 token_ttl=720h token_max_ttl=720h secret_id_ttl="0" secret_id_num_uses=0 token_policies="kv-celestus"

# Get role/secret id

vault read auth/approle/role/celestus/role-id
vault write -f auth/approle/role/celestus/secret-id

For more secure way wrap the secret_id:
vault write -wrap-ttl=10m -f auth/approle/role/celestus/secret-id
vault write -wrap-ttl=720h -f auth/approle/role/celestus/secret-id

# Login wtih approle

vault write auth/approle/login role_id="" secret_id=""

# Get secrets

vault kv get kv/dev/celestus/database/pg
