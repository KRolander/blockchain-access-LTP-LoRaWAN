#include "LightTransaction.h"
#include "payload.pb.h"
#include "pb_encode.h"
#include "ecdsa.h"
#include "secp256k1.h"

/* definitions errors */
#define NO_ERROR 1
#define ERROR_ENCODE_MESSAGE 2
#define ERROR_ENCODE_PAYLOAD 3

/**
 * Convert the error of build_light_payload function in text.
 *
 * @param error_code the int result of build_light_payload function
 * @return string
 */
char *printif_error(int error_code)
{
  switch (error_code)
  {
  case 1:
    return "No error";

  case 2:
    return "Encoding Message failed";

  case 3:
    return "Encoding Payload failed";

  default:
    return "Erreur undefined";
  }
}

/**
 * Generate the public key and the public .
 *
 * @param private_key pointer to an array of 64 Bytes key
 * @param public_key pointer to an array of 64 Bytes to write the public key before compression
 * @param public_key_compressed pointer to an array of 33 Bytes to write the public key compressed
 * @return true for sucess and false if failed
 */
bool generate_public_keys(byte private_key[], byte public_key_compressed[])
{
  const ecdsa_curve *curve = &secp256k1;
  if (!ecdsa_get_public_key33(curve, private_key, public_key_compressed))
    return true;
  else
    return false;
}

/**
 * Runs the three steps for hash one time with sha256 a data .
 *
 * @param data the message to hash
 * @param data_lenght length of the message
 * @param hashed_data pointer to an array of 32 Bytes to write the hash
 * @return void
 */
void hash_data(byte data[], int data_lenght, byte hashed_data[])
{
  SHA256_CTX sha256_ctx;
  sha256_Init(&sha256_ctx);
  sha256_Update(&sha256_ctx, data, data_lenght);
  sha256_Final(&sha256_ctx, hashed_data);
}

/**
 * Write into an array the signature.
 *
 * @param private_key pointer to an array of 64 Bytes key
 * @param hashed_data pointer to an array of hashed data of hash_size Bytes
 * @param signature pointer to an array of 65 Bytes to write the signature
 * @return void
 */
void sign_data(byte *private_key, byte *hashed_data, byte *signature)
{
  const ecdsa_curve *curve = &secp256k1;
   uint8_t pby;
  ecdsa_sign_digest(curve, private_key, hashed_data, signature, &pby, 0);
  signature[SIGNATURE_SIZE-1]=pby;
  //ecdsa_sign_digest(curve, private_key, hashed_data, signature, 0, 0); //for remove 65th bytes of signature
  
}

/**
 * Encode the differents fields of Message in a protobuff.
 *
 * @param protobuffer pointer of MESSAGE_SIZE_MAX Bytes array
 * @param protobuffer_length pointer to a size_t value to write protobuffer size
 * @param my_message Message struct(typedef define in payload.pb.h)
 * @return true for sucess and false if failed
 */
bool encoding_protobuf(byte protobuffer[], size_t *protobuffer_length, const Message my_message)
{
  bool status;
  pb_ostream_t stream;

  stream = pb_ostream_from_buffer(protobuffer, MESSAGE_SIZE_MAX); /* Constructs an output stream for writing into a memory buffer */
  status = pb_encode(&stream, Message_fields, &my_message);       /* Encodes the contents of a structure as a protocol buffers message and writes it to output stream */
  *protobuffer_length = stream.bytes_written;
  if (!status)
  {

    Serial.print("Encode Message in protobuffer error, type: "); // Use for debug
    Serial.println(PB_GET_ERROR(&stream));
    return false;
  }
  else
  {
    return true;
  }
}

/**
 * Encode the differents fields of Payload in a protobuff.
 *
 * @param protobuffer pointer of LORA_NETWORK_MAX Bytes array
 * @param protobuffer_length pointer to a size_t value to write protobuffer size
 * @param my_paylaod Payload struct (typedef define in payload.pb.h)
 * @return true for sucess and false if failed
 */
bool encoding_protobuf(byte protobuffer[], size_t *protobuffer_length, const Payload my_payload)
{
  bool status;
  pb_ostream_t stream;

  stream = pb_ostream_from_buffer(protobuffer, NETWORK_SIZE_MAX); /* Constructs an output stream for writing into a memory buffer */
  status = pb_encode(&stream, Payload_fields, &my_payload);       /* Encodes the contents of a structure as a protocol buffers message and writes it to output stream */
  *protobuffer_length = stream.bytes_written;
  if (!status)
  {

    Serial.print("Encode Payload in protobuffer error, type: "); // Use for debug
    Serial.println(PB_GET_ERROR(&stream));
    return false;
  }
  else
  {
    return true;
  }
}

/**
 * Manage the creation of a light payload.
 *
 * @param private_key pointer to an array of 64 bytes private key
 * @param public_key_compressed pointer to an array of public key 33 bytes long
 * @param my_message Message (typedef struct define in payload.pb.h)
 * @param protobuffer array of 242 bytes to write the light serialised payload
 * @param protobuffer_length pointer to write the protobuf size
 * @return int value corresponding to a specific error
 */

int build_light_payload(byte private_key[], byte public_key_compressed[],
                        Message my_message, byte protobuffer[],
                        size_t *protobuffer_length)
{
  //Serial3.write("3\n");
  byte my_protobuffer_message_only[MESSAGE_SIZE_MAX]; // TODO: find best names
  size_t protobuffer_length_message_only;

  if (!encoding_protobuf(my_protobuffer_message_only, &protobuffer_length_message_only, my_message)) // Serialised Message
  {
    return ERROR_ENCODE_MESSAGE;
  }
 // Serial3.write("3E\n");
 // Serial3.write("4\n");
  byte hash[SHA256_DIGEST_LENGTH];
  hash_data(my_protobuffer_message_only, protobuffer_length_message_only, hash); // Hash Message serialised
  //Serial3.write("4E\n");
  //Serial3.write("5\n");
  byte signature[SIGNATURE_SIZE];
  sign_data(private_key, hash, signature); // Sign Message serialised
  //print_array(signature,65);
//Serial3.write("5E\n");
//Serial3.write("6\n");
  Payload my_payload = {};
  my_payload.message = my_message; //*Luc* est ce que je crée une fnct à part pour initialisé les champs? TODO: Create a function for init fields ?
  memcpy(my_payload.signature.bytes, signature, SIGNATURE_SIZE);
  my_payload.signature.size = SIGNATURE_SIZE;
  memcpy(my_payload.public_key.bytes, public_key_compressed, PUBLIC_KEY_COMPRESSED_SIZE);
  my_payload.public_key.size = PUBLIC_KEY_COMPRESSED_SIZE;

  if (encoding_protobuf(protobuffer, protobuffer_length, my_payload)) // Serialised Paylaod
  {
    //Serial3.write("6E\n");
    return NO_ERROR;
  }
  else
  {
    return ERROR_ENCODE_PAYLOAD;
  }
}

void print_array(uint8_t *myarray, int mysize) // Use for debug
{
  char buffer[3];
  for (int i = 0; i < mysize; i++)
  {
    sprintf(buffer, "%02x", myarray[i]);
    Serial.print(buffer);
  }
  Serial.println("");
}