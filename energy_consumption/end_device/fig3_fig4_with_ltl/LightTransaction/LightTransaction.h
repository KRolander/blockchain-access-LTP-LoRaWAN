#ifndef _LightTransaction_H_
#define _LightTransaction_H_
#endif

#include <Arduino.h>
#include "sha2.h"
#include "payload.pb.h"

#define PUBLIC_KEY_COMPRESSED_SIZE 33
#define SIGNATURE_SIZE 65
#define HASH_SIZE 32
#define SHA256_BLOCK_LENGTH 64 
#define SHA256_DIGEST_LENGTH 32
#define PRIVATE_KEY_SIZE 32
#define NETWORK_SIZE_MAX 242
#define MESSAGE_SIZE_MAX 160 

bool generate_public_keys(byte private_key[], byte public_key_compressed[]);

void hash_data(byte data[], int data_lenght, byte my_hashed_data[]);

void sign_data(byte *private_key, byte *hashed_data, byte *signature);

bool encoding_protobuf(byte protobuffer[], size_t *protobuffer_length,const Message my_message);

bool encoding_protobuf(byte protobuffer[], size_t *protobuffer_length, const Payload my_payload);

int build_light_payload(byte private_key[], byte public_key_compressed[],
                        Message my_message, byte protobuffer[],
                        size_t *protobuffer_length);

char *printif_error(int error_code);

void print_array(uint8_t * myarray, int mysize);
