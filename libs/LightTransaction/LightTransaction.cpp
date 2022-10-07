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
 * @return string to print
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
 * Generate the public key compressed.
 *
 * @param private_key 32 bytes array containing the private key
 * @param public_key_compressed 33 bytes array to write the public key compressed
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
 * Runs the 3 steps to make a Sha256 hash.
 *
 * @param data the message to hash
 * @param data_lenght length of the message to hash
 * @param hashed_data 32 bytes array to write the hash
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
 * @param private_key 32 bytes array containing the private key
 * @param hashed_data 32 bytes array containing the hashed data
 * @param signature 65 bytes array to write the signature
 * @return void
 */
void sign_data(byte *private_key, byte *hashed_data, byte *signature)
{
  const ecdsa_curve *curve = &secp256k1;
  uint8_t pby;
  ecdsa_sign_digest(curve, private_key, hashed_data, signature, &pby, 0);
  signature[SIGNATURE_SIZE - 1] = pby; // comments to remove the 65th byte
  // ecdsa_sign_digest(curve, private_key, hashed_data, signature, 0, 0); //for remove 65th byte of signature
}

/**
 * Encode the differents fields of Message in a protobuff.
 *
 * @param protobuffer MESSAGE_SIZE_MAX bytes array to write the proto-message
 * @param protobuffer_length size_t value to write proto-message size
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

    // Serial.print("Encode Message in protobuffer error, type: "); // Use for debug
    // Serial.println(PB_GET_ERROR(&stream));
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
 * @param protobuffer LORA_NETWORK_MAX bytes array to write the proto-payload
 * @param protobuffer_length size_t value to write proto-payload size
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

    // Serial.print("Encode Payload in protobuffer error, type: "); // Use for debug
    // Serial.println(PB_GET_ERROR(&stream));
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
 * @param private_key 32 bytes array containing the private key
 * @param public_key_compressed 33 bytes array containing the public key compressed
 * @param my_message Message (typedef struct define in payload.pb.h)
 * @param protobuffer array of LORA_NETWORK_MAX bytes to write the light serialised payload
 * @param protobuffer_length size_t value to write proto-message size
 * @return int value corresponding to a specific error
 */
int build_light_payload(byte private_key[], byte public_key_compressed[],
                        Message my_message, byte protobuffer[],
                        size_t *protobuffer_length)
{

  byte my_protobuffer_message_only[MESSAGE_SIZE_MAX];
  size_t protobuffer_length_message_only;

  if (!encoding_protobuf(my_protobuffer_message_only, &protobuffer_length_message_only, my_message)) // Serialised Message
  {
    return ERROR_ENCODE_MESSAGE;
  }

  byte hash[SHA256_DIGEST_LENGTH];
  hash_data(my_protobuffer_message_only, protobuffer_length_message_only, hash); // Hash Message serialised

  byte signature[SIGNATURE_SIZE];
  sign_data(private_key, hash, signature); // Sign Message serialised

  // Init payload fiels
  Payload my_payload = {};
  my_payload.message = my_message;
  memcpy(my_payload.signature.bytes, signature, SIGNATURE_SIZE);
  my_payload.signature.size = SIGNATURE_SIZE;
  memcpy(my_payload.public_key.bytes, public_key_compressed, PUBLIC_KEY_COMPRESSED_SIZE);
  my_payload.public_key.size = PUBLIC_KEY_COMPRESSED_SIZE;

  if (encoding_protobuf(protobuffer, protobuffer_length, my_payload)) // Serialised Payload
  {
    return NO_ERROR;
  }
  else
  {
    return ERROR_ENCODE_PAYLOAD;
  }
}

/**
 * Quick display of byte array in debug mode.
 *
 * @param myarray array for display
 * @param mysize size of myarray
 * @return void
 */
void print_array(uint8_t *myarray, int mysize)
{
  char buffer[3];
  for (int i = 0; i < mysize; i++)
  {
    sprintf(buffer, "%02x", myarray[i]);
    Serial.print(buffer);
  }
  Serial.println("");
}