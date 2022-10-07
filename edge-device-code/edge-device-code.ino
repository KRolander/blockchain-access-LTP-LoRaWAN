#ifndef REGION
#define REGION R_EU868
#endif

#include "RelayConfig.h"
#include <LibLacuna.h>

#include <SPI.h>
#include <time.h>
#include <EEPROM.h>
#include <RTC.h>

// Global variables
extern uint32_t relay_device_list[];
extern uint8_t *relay_nwk_key[];
lsRelayParams_t relayConfig;

// Relay's status payload
static char payload[255];
static int payloadLength;

// Device's payload
static byte relay_payload[255];
lsRelayResultInfo_t result;

uint8_t relay_dev_not_in_list = 0;
uint8_t relay_packet_mic_fail = 0;
uint8_t relay_invalid_frm_cnt = 0;
uint16_t relay_uplinks = 0;

extern volatile int preamble_detect;

lsLoraTxParams txParams;
lsLoraTxParams relayParams;


void setup()
{
#ifdef ENABLE_LED
  pinMode(LS_LED_BLUE, OUTPUT);
#endif

  Serial.begin(9600);
  while (!Serial && millis() < 3000);
  Serial.println("Initializing...");
  analogReadResolution(12);

  lsSX126xConfig cfg;
  lsCreateDefaultSX126xConfig(&cfg);
  int result = lsInitSX126x(&cfg); //return 0 if no error

  if (result)
    Serial.println(lsErrorToString(result));

  /* LoRa parameters for device (forward) & relay status */
  lsCreateDefaultLoraTxParams(&txParams);
  txParams.power = 14;
  txParams.spreadingFactor = lsLoraSpreadingFactor_7;
  txParams.codingRate = lsLoraCodingRate_4_5;
  txParams.invertIq = false;
  txParams.frequency = 868100000;
  txParams.bandwidth = lsLoraBandwidth_125_khz;
  txParams.syncWord = LS_LORA_SYNCWORD_PUBLIC;
  txParams.preambleLength = 8;

  /* LoRa parameters for relay (receive) */
  lsCreateDefaultLoraTxParams(&relayParams);
  relayParams.spreadingFactor = lsLoraSpreadingFactor_7;
  relayParams.invertIq = false;
  relayParams.frequency = 865000000;
  relayParams.bandwidth = lsLoraBandwidth_125_khz;
  relayParams.syncWord = LS_LORA_SYNCWORD_PUBLIC;

  /* Relay config */
  //lsCreateRelayParams(&relayConfig, RELAY_MODE, NUM_OF_DEVICE_IN_WHITELIST, relay_device_list, relay_nwk_key);
  lsCreateRelayParams(&relayConfig, lsRelayPacketCheck_Disable, NUM_OF_DEVICE_IN_WHITELIST, relay_device_list, relay_nwk_key);

  Serial.print("Relay Initialization DONE\n");
  // LED_Blink(2, 200);
  delay(500);
}

void loop()
{
#ifdef LOWPOWER
  uint32_t rxlength = lsRelayLora2(&relayParams, relay_payload, LS200_Sleep, &relayConfig, &result);
#else
  uint32_t rxlength = lsRelayLora2(&relayParams, relay_payload, NULL, &relayConfig, &result);
#endif
  //Serial.println(lsRelayStatusTypeToString(result.status));
  if (rxlength && (result.status == lsRelayStatus_OK))/* valid relay data received */
  {
    /* Relay the message */
    Serial.print("Message length is: "); Serial.println(rxlength);
    //Serial.print("Messgae is : "); print_array(relay_payload, rxlength);
    int lora_result = lsSendLora(&txParams, (byte *)relay_payload, rxlength, false);
    //Serial.print("LoRa TX Relay Result: ");Serial.println(lsErrorToString(lora_result));
  }
  else
  {
    switch (result.status)
    {
      case lsRelayStatus_DevNotInList:
        relay_dev_not_in_list++;
        break;
      case lsRelayStatus_PacketMICFail:
        relay_packet_mic_fail++;
        break;
      case lsRelayStatus_InvalidFrmCnt:
        relay_invalid_frm_cnt++;
        break;
      default:
        break;
    }
    //Serial.println(lsRelayStatusTypeToString(result.status)); // Print error
  }
}

void LS200_Sleep()
{
  SPI.end();
  STM32.stop();
  SPI.begin();
}
