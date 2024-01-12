# Genrating and using secrets

## Generate public and secre keys

`gpg --full-gen-key` \
`gpg --full-generate-key`

## List them

`gpg --list-public-keys` \
`gpg --list-secret-keys` \
`gpg --list-keys --keyid-format short` \
`gpg --list-keys --keyid-format long` \
`gpg --list-public-keys --fingerprint --with-subkey-fingerprints --keyid-format 0xlong`

## Export/Import them

### Locally

`gpg --output pubkey.gpg --export SOMEKEYID && \
gpg --output - --export-secret-key SOMEKEYID |\
    cat pubkey.gpg - |\
    gpg --armor --output keys.asc --symmetric --cipher-algo AES256` \

And then import from the resulting keys.asc file: \
`gpg --output - keys.asc | gpg --impor`

### Remotely

If you’re on the machine that already has the key: \
`gpg --export-secret-key SOMEKEYID | ssh othermachine gpg --import`

If you’re on the machine that needs the key: \
`ssh othermachine gpg --export-secret-key SOMEKEYID | gpg --import`

## Encrypt

### Files

Using public key: \
`gpg --encrypt -r <e-mail@from.key> file_to_encrypt.txt`

Using password: \
`gpg --output file.gpg --symmetric file_to_encrypt`

### Strings

Using public key: \
`echo "text_to_encrypt" | gpg --encrypt --armor -r <e-mail@from.key>`

Using password:

- To console: \
`echo "text_to_encrypt" | gpg --armor --symmetric --cipher-algo AES256`

- To file: \
`echo "text_to_encrypt" | gpg --armor --output keys.asc --symmetric --cipher-algo AES256`

## Decrypt

Using private key: \
`gpg --decrypt message.txt.gpg > message.txt`

Using password: \
`gpg --decrypt message.txt.gpg`
