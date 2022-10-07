#ifndef REGION
#define REGION R_EU868
#endif


#include "RelayConfig.h"
#include <LibLacuna.h>

#include <SPI.h>
#include <time.h>
#include <EEPROM.h>
#include <RTC.h>
#define  LOWPOWER

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

    Serial3.begin(115200);
   while (!Serial3 && millis() < 5000);
  //Serial.println("Initializing...");
  analogReadResolution(12);

  lsSX126xConfig cfg;
  lsCreateDefaultSX126xConfig(&cfg);
  int result = lsInitSX126x(&cfg); //return 0 if no error
  
  if (result)
   // Serial.println(lsErrorToString(result));

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
  lsCreateRelayParams(&relayConfig, lsRelayPacketCheck_Disable, NUM_OF_DEVICE_IN_WHITELIST, relay_device_list, relay_nwk_key);       //lsRelayPacketCheck:Disable,AddrFilterOnly,sRelayPacketCheck_MICCheck,

  //Serial.print("Relay Initialization DONE\n");
 // LED_Blink(2, 200);
  delay(500);
}

void loop()
{
  Serial3.write("0\n");
#ifdef LOWPOWER
  uint32_t rxlength = lsRelayLora2(&relayParams, relay_payload, LS200_Sleep, &relayConfig, &result);
#else
  uint32_t rxlength = lsRelayLora2(&relayParams, relay_payload, NULL, &relayConfig, &result);
#endif
Serial3.write("1E\n");
//Serial3.println(lsRelayStatusTypeToString(result.status));
  if (rxlength && (result.status == lsRelayStatus_OK))/* valid relay data received */
  { 
  //  LED_Blink(4, 100);

    /* Relay the message */
Serial3.write("2\n");
//Serial3.print("Mon proto est long de: "); 
//Serial3.println(rxlength);
// Serial3.print("Mon proto est : "); print_array(relay_payload, rxlength);
    int lora_result = lsSendLora(&txParams, (byte *)relay_payload, rxlength, false);
   Serial3.write("2E\n");
   //Serial3.print("LoRa TX Relay Result: ");
  //  Serial3.println(lsErrorToString(lora_result));
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
    //Serial3.println(lsRelayStatusTypeToString(result.status)); // Print error
    Serial3.write("E");
  }
}


void LS200_Sleep()
{
  SPI.end();
  STM32.stop();
  SPI.begin();
}


inline void LED_Blink(uint8_t num_of_blink, uint16_t time_on)
{
#ifdef ENABLE_LED
  for (int i = 0; i < num_of_blink; i++)
  {
    digitalWrite(LS_LED_BLUE, HIGH);
    delay(time_on);
    digitalWrite(LS_LED_BLUE, LOW);
    delay(100);
  }
#endif
}
void print_array(uint8_t *myarray, int mysize) // Use for debug
{
  char buffer[3];
  for (int i = 0; i < mysize; i++)
  {
    sprintf(buffer, "%02x", myarray[i]);
    Serial3.print(buffer);
  }
  Serial.println("");
}
