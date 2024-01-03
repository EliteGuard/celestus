#!/bin/bash

##
# General OpenSSL Commands
#
# These commands allow you to generate CSRs, Certificates,
# Private Keys and do other miscellaneous tasks.
##

DOMAIN_NAME=$1
COMMAND=$2

if [ -z "${DOMAIN_NAME}" ]
then
    read -p "Please enter a domain name: "
    DOMAIN_NAME=$REPLY

    if [ -z "${DOMAIN_NAME}" ]
    then
        echo "A domain name is required. Aborting ... "
        echo ""
        ossl_help
        echo ""
        exit 1
    fi
fi

# confirm if user wants to continue ...
read -p "Continue to perform SSL operations for ${DOMAIN_NAME}? "
if [[ ! $REPLY =~ ^[Yy]$ ]]
then
    echo "Aborted"
    exit 0
fi

if [ -z "${COMMAND}" ]
then
    ossl_help
    echo ""

    read -p "Please enter a valid command: "
    COMMAND=$REPLY

    if [ -z "${COMMAND}" ]
    then
        echo "A command is required. Aborting ... "
        echo ""
        exit 1
    fi
fi

echo ""

CACERT="CACert.crt"
CSR="${DOMAIN_NAME}.csr"
CRT="${DOMAIN_NAME}.crt"
KEY="${DOMAIN_NAME}.key"
DER="${DOMAIN_NAME}.der"
PEM="${DOMAIN_NAME}.pem"
PFX="${DOMAIN_NAME}.pfx"

DAYS=365 #730 # 2 years.

function ossl_help {
    PREFIX="~/ssl.sh domain_name command"
    echo "$PREFIX [type]"
    echo "Examples: "
    echo "    ~/ssl.sh example.com csr"
    echo "    ~/ssl.sh example.com crt"
    echo "    ~/ssl.sh example.com key"
    echo "    ~/ssl.sh example.com connect"
    echo "    ~/ssl.sh example.com inspect [csr|crt|key|pfx]"
    echo "    ~/ssl.sh example.com hash [csr|crt|key|all]"
    echo "    ~/ssl.sh example.com convert [der2pem|pem2der|pfx2pem|pem2pfx]"
}

function ossl_hash_csr {
    openssl req -noout -modulus -in $CSR | openssl md5
}
function ossl_hash_key {
    openssl rsa -noout -modulus -in $KEY | openssl md5
}
function ossl_hash_crt {
    openssl x509 -noout -modulus -in $CRT | openssl md5
}
function ossl_hash_all {
    ossl_hash_csr
    ossl_hash_key
    ossl_hash_crt
}

function ossl_inspect_csr {
    openssl req -verify -text -noout -in $CSR
}
function ossl_inspect_key {
    openssl rsa -check -in $KEY
}
function ossl_inspect_crt {
    openssl x509 -text -noout -in $CRT
}
function ossl_inspect_pfx {
    # Check a PKCS#12 file (.pfx or .p12)
    openssl pkcs12 -info -in $PFX
}

function ossl_csr {
    if [ "key_crt" = "$1" ]]
    then
        openssl x509 -x509toreq -signkey $KEY -in $CRT -out $CSR
    else
        SIGNING="-nodes -newkey rsa:2048 -keyout" # key_new|key_existing|key_crt

        if [ "key_existing" = "$1" ]
        then
            SIGNING="-key"
        fi

        openssl req -new -sha256 $SIGNING $KEY -out $CSR
    fi
}

function ossl_key_passphrase_remove {
    openssl rsa -in $KEY -out "nopassphrase-$KEY"
}

function ossl_crt_self_signed {
    SIGNING="-newkey rsa:2048 -keyout" # key_new|key_existing

    if [ "key_existing" = "$1" ]
    then
        SIGNING="-signkey"
    fi

    openssl x509 -req -days $DAYS $SIGNING $KEY -in $CSR -out $CRT
}

function ossl_connect {
    openssl s_client -connect $1:443
}

function ossl_convert_der_to_pem {
    openssl x509 -inform der -in $DER -out $PEM
}
function ossl_convert_pem_to_der {
    openssl x509 -outform der -in $PEM -out $DER
}
function ossl_convert_pem_to_pfx {
    # Convert a PEM certificate file and a private key to PKCS#12 (.pfx .p12)
    openssl pkcs12 -export -inkey $KEY -certfile $CACERT -in $PEM -out $PFX
}
function ossl_convert_pfx_to_pem {
    EXCLUDE="" # -nokeys | -nocerts

    if [ -n "$1" ] # if set to a non-empty string.
    then
        EXCLUDE="-$1" # prefix with hyphen.
    fi

    # convert a PKCS#12 file (.pfx .p12) containing a private key and certificates to PEM.
    openssl pkcs12 -nodes -in $PFX -out $PEM $EXCLUDE
}


case $COMMAND in
    csr )
        echo "Generate Certificate Signing Request (CSR) to $CSR"

        read -p "Do you want to use an existing certificate and private key ($CRT)? "
        #read -p "Are you sure? " -n 1 -r
        #echo "Selected: $REPLY" # (optional) move to a new line
        if [[ $REPLY =~ ^[Yy]$ ]]
        then
            echo "Converting an existing certificate ($CRT) and private key ($KEY) to CSR."
            ossl_csr key_crt
        else
            read -p "Do you want to use just an existing private key ($KEY)? "
            if [[ $REPLY =~ ^[Yy]$ ]]
            then
                echo "Generating a CSR using existing private key."
                ossl_csr key_existing
            else
                echo "Generating a CSR with a new private key."
                ossl_csr key_new
            fi
        fi
    ;;

    key )
        read -p "Do you want remove the passphrase from the private key ($KEY)? "
        if [[ $REPLY =~ ^[Yy]$ ]]
        then
            echo "Removing passphrase from private key"
            ossl_key_passphrase_remove
        fi
    ;;

    crt )
        read -p "Do you want a self-signed certificate ($CRT)? "
        if [[ $REPLY =~ ^[Yy]$ ]]
        then
            read -p "Do you want to use an existing key for your self-signed certificate ($KEY)? "
            if [[ $REPLY =~ ^[Yy]$ ]]
            then
                echo "Generate a self-signed certificate using existing private key."
                ossl_crt_self_signed key_existing
            else
                echo "Generate a self-signed certificate with a new private key."
                ossl_crt_self_signed key_new
            fi
        fi
    ;;

    # Inspect Using OpenSSL
    inspect )
        TYPE=$3

        if [ -z "${TYPE}" ]
        then
            read -p "Which file do you want to insepct [csr|key|crt|pfx]: "
            TYPE=$REPLY
        fi

        case $TYPE in
            csr )
                echo "Inspecting the CSR ($CSR)"
                ossl_inspect_csr
            ;;
            key )
                echo "Inspecting the private key ($KEY)"
                ossl_inspect_key
            ;;
            crt )
                echo "Inspecting the certificate ($CRT)"
                ossl_inspect_crt
            ;;
            pfx )
                echo "Inspecting the PFX/PKCS#12 ($PFX)"
                ossl_inspect_pfx
            ;;
        esac
    ;;

    # Debugging Using OpenSSL
    hash )
        # Check an MD5 hash of the public key to ensure that it matches with what
        # is in a CSR or private key
        TYPE=$3

        if [ -z "${TYPE}" ]
        then
            read -p "Which file do you want to generate a hash for [csr|key|crt|all]: "
            TYPE=$REPLY
        fi

        case $TYPE in
            csr )
                echo "Generating MD5 hash of CSR ($CSR)"
                ossl_hash_csr
            ;;
            key )
                echo "Generating MD5 hash of private key ($KEY)"
                ossl_hash_key
            ;;
            crt )
                echo "Generating MD5 hash of certificate ($CRT)"
                ossl_hash_crt
            ;;
            all )
                echo "Generating MD5 hashes for certificate, private key and CSR."
                ossl_hash_all
            ;;
            * )
                echo "Generating MD5 hashes for certificate, private key and CSR."
                ossl_hash_all
            ;;
        esac
    ;;

    connect )
        echo "Checking SSL connection at host: $DOMAIN_NAME"
        ossl_connect $DOMAIN_NAME
    ;;

    # Converting Using OpenSSL
    convert )
        TYPE=$3

        if [ -z "${TYPE}" ]
        then
            read -p "Which coversion do you want to perform [der2pem|pem2der|pfx2pem|pem2pfx]: "
            TYPE=$REPLY
        fi

        case $TYPE in
            der2pem )
                echo "Converting DER ($DER) to PEM ($PEM)"
                ossl_convert_der_to_pem
            ;;
            pem2der )
                echo "Converting PEM ($PEM) to DER ($DER)"
                ossl_convert_pem_to_der
            ;;
            pfx2pem )
                echo "Converting PFX ($PFX) to PEM ($PEM)"

                read -p "Extract both private key and certificates to PEM? "
                if [[ $REPLY =~ ^[Yy]$ ]]
                then
                    ossl_convert_pfx_to_pem
                else
                    read -p "Extract only the certificates (-nokeys) to PEM ($PEM)? "
                    if [[ $REPLY =~ ^[Yy]$ ]]
                    then
                        ossl_convert_pfx_to_pem nocerts
                    fi

                    read -p "Extract only the private key (-nocerts) to PEM? "
                    if [[ $REPLY =~ ^[Yy]$ ]]
                    then
                        ossl_convert_pfx_to_pem nokeys
                    fi
                fi
            ;;
            pem2pfx )
                echo "Converting PEM ($PEM) to PFX ($PFX)"

                read -p "Do you have a -certfile ($CACERT) "
                if [[ $REPLY =~ ^[Yy]$ ]]
                then
                    ossl_convert_pem_to_pfx
                fi
            ;;
        esac
    ;;

    help )
        ossl_help
    ;;

    * )
        ossl_help
    ;;
esac

echo ""
echo "Done."
echo ""

exit 0