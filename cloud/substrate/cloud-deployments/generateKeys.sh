#!/bin/bash

NB_KEYS=30
KEYS_FILE=keys_file.sh

####################################Ed25519
Ed25519_arr_secretPhrase=()
Ed25519_arr_ss58PublicKey=()
Ed25519_arr_ss58Address=()
Ed25519_arr_publicKey=()
Ed25519_arr_accountId=()
Ed25519_arr_secretSeed=()

for (( i=0; i<=$NB_KEYS; i++ ))
do 

    data=$(subkey generate --scheme Ed25519 --output-type json)

    Ed25519_arr_secretPhrase+=("$(echo $data | jq -r '.secretPhrase')")
    Ed25519_arr_ss58PublicKey+=("$(echo $data | jq -r ".ss58PublicKey")")
    Ed25519_arr_ss58Address+=("$(echo $data | jq -r ".ss58Address")")
    Ed25519_arr_publicKey+=("$(echo $data | jq -r ".publicKey")")
    Ed25519_arr_accountId+=("$(echo $data | jq -r ".accountId")")
    Ed25519_arr_secretSeed+=("$(echo $data | jq -r ".secretSeed")")

done

####################################Sr25519
Sr25519_arr_secretPhrase=()
Sr25519_arr_ss58PublicKey=()
Sr25519_arr_ss58Address=()
Sr25519_arr_publicKey=()
Sr25519_arr_accountId=()
Sr25519_arr_secretSeed=()

for (( i=0; i<=$NB_KEYS; i++ ))
do 

    data=$(subkey inspect "${Ed25519_arr_secretPhrase[i]}" --scheme Sr25519 --output-type json)

    Sr25519_arr_secretPhrase+=("$(echo $data | jq -r ".secretPhrase")")
    Sr25519_arr_ss58PublicKey+=("$(echo $data | jq -r ".ss58PublicKey")")
    Sr25519_arr_ss58Address+=("$(echo $data | jq -r ".ss58Address")")
    Sr25519_arr_publicKey+=("$(echo $data | jq -r ".publicKey")")
    Sr25519_arr_accountId+=("$(echo $data | jq -r ".accountId")")
    Sr25519_arr_secretSeed+=("$(echo $data | jq -r ".secretSeed")")

done


echo "###########################################################" > $KEYS_FILE
echo "export Ed25519_arr_secretPhrase=($(printf "\"%s\" "  "${Ed25519_arr_secretPhrase[@]}"))" >> $KEYS_FILE
echo "export Ed25519_arr_ss58PublicKey=($(printf "\"%s\" "  "${Ed25519_arr_ss58PublicKey[@]}"))" >> $KEYS_FILE
echo "export Ed25519_arr_ss58Address=($(printf "\"%s\" "  "${Ed25519_arr_ss58Address[@]}"))" >> $KEYS_FILE
echo "export Ed25519_arr_publicKey=($(printf "\"%s\" "  "${Ed25519_arr_publicKey[@]}"))" >> $KEYS_FILE
echo "export Ed25519_arr_accountId=($(printf "\"%s\" "  "${Ed25519_arr_accountId[@]}"))" >> $KEYS_FILE
echo "export Ed25519_arr_secretSeed=($(printf "\"%s\" "  "${Ed25519_arr_secretSeed[@]}"))" >> $KEYS_FILE
echo "###########################################################" >> $KEYS_FILE
echo "export Sr25519_arr_secretPhrase=($(printf "\"%s\" "  "${Sr25519_arr_secretPhrase[@]}"))" >> $KEYS_FILE
echo "export Sr25519_arr_ss58PublicKey=($(printf "\"%s\" "  "${Sr25519_arr_ss58PublicKey[@]}"))" >> $KEYS_FILE
echo "export Sr25519_arr_ss58Address=($(printf "\"%s\" "  "${Sr25519_arr_ss58Address[@]}"))" >> $KEYS_FILE
echo "export Sr25519_arr_publicKey=($(printf "\"%s\" "  "${Sr25519_arr_publicKey[@]}"))" >> $KEYS_FILE
echo "export Sr25519_arr_accountId=($(printf "\"%s\" "  "${Sr25519_arr_accountId[@]}"))" >> $KEYS_FILE
echo "export Sr25519_arr_secretSeed=($(printf "\"%s\" "  "${Sr25519_arr_secretSeed[@]}"))" >> $KEYS_FILE
echo "###########################################################" >> $KEYS_FILE
