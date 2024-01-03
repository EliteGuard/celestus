#!/usr/bin/env sh

set -ex

unseal () {
    vault operator unseal $(grep 'Key 1:' /vault/data/keys | awk '{print $NF}')
    vault operator unseal $(grep 'Key 2:' /vault/data/keys | awk '{print $NF}')
    vault operator unseal $(grep 'Key 3:' /vault/data/keys | awk '{print $NF}')
}

init () {
    vault operator init > /vault/data/keys
}

log_in () {
   export ROOT_TOKEN=$(grep 'Initial Root Token:' /vault/data/keys | awk '{print $NF}')
   vault login $ROOT_TOKEN
}

create_token () {
   vault token create -id $DEV_VAULT_TOKEN
}

if [ -s /vault/data/keys ]; then
   unseal
else
   init
   unseal
   log_in
   create_token
fi

vault status > /vault/data/status