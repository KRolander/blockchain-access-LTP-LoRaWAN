/*
  Pour tester si le fonctionnement est ok: https://paulmillr.com/noble/
  Private key de test: ec3c161597b119998ad8822acb2b1123405e85c7577e5e8abd678ad8c2383f2c
  (= echo -n luc | sha256sum)
  by Luc
*/

#include <LightTransaction.h>
#include "payload.pb.h"
/*
  Define your message here
*/
#define DATA_SIZE 5 //max_size is 121
byte b_data[DATA_SIZE] = {0x6d, 0x61, 0x6e, 0x6f, 0x6e};

/*
  Define your smart contract target name here
*/
#define SMART_CONTRACT_SIZE 4 //max_size is 12
byte smart_contract[SMART_CONTRACT_SIZE] = {0x69, 0x6f, 0x74, 0x62};

/*
  Selects the target blockchain with BLOCKCHAIN_ID:
  Blockchain_HYPERLEDGER_FABRIC
  Blockchain_HYPERLEDGER_SAWTOOTH
  Blockchain_SUBSTRATE
  Blockchain_ETHEREUM
*/
#define BLOCKCHAIN_ID Blockchain_SUBSTRATE

/*
  Selects the smart contract action with ACTION
  Action_SET_DATA
  Action_GET_DATA
*/
#define   ACTION Action_GET_DATA


byte my_protobuffer[242];
size_t protobuffer_length;

byte private_key[PRIVATE_KEY_SIZE] = {0xec, 0x3c, 0x16, 0x15, 0x97, 0xb1, 0x19, 0x99, 0x8a, 0xd8, 0x82, 0x2a, 0xcb, 0x2b, 0x11, 0x23, 0x40, 0x5e, 0x85, 0xc7, 0x57, 0x7e, 0x5e, 0x8a, 0xbd, 0x67, 0x8a, 0xd8, 0xc2, 0x38, 0x3f, 0x2c};
byte public_key_compressed[PUBLIC_KEY_COMPRESSED_SIZE] = {};

void setup() {
  Serial.begin(9600);
  while (!Serial && millis() < 2000);

  int resultpub = generate_public_keys(private_key, public_key_compressed);
  if (!resultpub)
  {
    Serial.print("Generate public keys failled");
  }
  else
  {
    Serial.print("Public key: ");
    print_array(public_key_compressed, PUBLIC_KEY_COMPRESSED_SIZE);
  }
}

void print_array(uint8_t * myarray, int mysize)
{
  char buffer[3];
  for (int i = 0; i < mysize; i++) {
    sprintf (buffer, "%02x", myarray[i]);
    Serial.print(buffer);
  }
  Serial.println("");
}



void loop() {

  int  a = millis();
  int error = build_light_payload(private_key, public_key_compressed, b_data,  DATA_SIZE,
                                  blockchain_id, action, smart_contract,SMART_CONTRACT_SIZE, my_protobuffer,
                                  &protobuffer_length);
  int  b = millis();
  Serial.print("Result of building : "); Serial.println( printif_error(error));
  Serial.print("Protobuffer length:"); Serial.println(protobuffer_length);
  Serial.print("Protobuffer : ");
  print_array(my_protobuffer, protobuffer_length);

  Serial.print("Excecution time : "); Serial.print(b - a); Serial.println("ms");
  Serial.println("");
  delay(1000);

}
