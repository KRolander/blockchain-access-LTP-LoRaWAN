/*
  Pour tester si le fonctionnement est ok: https://paulmillr.com/noble/
  Private key de test: ec3c161597b119998ad8822acb2b1123405e857577e5e8abd678ad8c2383f20c
  by Luc
*/

#include <LightTransaction.h>
#include "payload.pb.h"

Message my_message; //Protobuffer message 
byte my_protobuffer[NETWORK_SIZE_MAX]; //In accordance with the maximum payload size in LoRaWAN. You can modify "NETWORK_SIZE_MAX" in LightTransaction.h file
size_t protobuffer_length;
byte private_key[PRIVATE_KEY_SIZE] = {0xec , 0x3c , 0x16 , 0x15 , 0x97 , 0xb1 , 0x19 , 0x99 , 0x8a , 0xd8 , 0x82 , 0x2a , 0xcb , 0x2b , 0x11 , 0x23 , 0x40 , 0x5e , 0x85 , 0x75 , 0x77 , 0xe5 , 0xe8 , 0xab , 0xd6 , 0x78 , 0xad , 0x8c , 0x23 , 0x83 , 0xf2 , 0x0c};
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


void generate_message(void)
{
  float temperature, altitude, pressure;
  float x, y, z;

  x = 1.0;
  y = 1.1;
  z = 1.2;

  temperature = 25.5;
  altitude = 101.5;
  pressure = 30.0;

  Sensors_uca_board my_sensors = {
    temperature,
    altitude,
    pressure,
    x,y,z
  };

  my_message = {
    Blockchain_SUBSTRATE,
    {0x69, 0x6f, 0x74, 0x62}, //"iotb"
    Action_GET_DATA,
    my_sensors
  };

}

void loop() {

  generate_message();

  int  a = millis();
  int error = build_light_payload(private_key, public_key_compressed, my_message, my_protobuffer, 
              &protobuffer_length);
  int  b = millis();
  Serial.print("Result of LTL : "); Serial.println( printif_error(error));
  Serial.print("Protobuffer length:"); Serial.println(protobuffer_length);
  Serial.print("Protobuffer : "); print_array(my_protobuffer, protobuffer_length);

  Serial.print("Excecution time : "); Serial.print(b - a); Serial.println("ms");
  Serial.println("");
  delay(1000);

}
