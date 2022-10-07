/*
  Pour tester si le fonctionnement est ok: https://paulmillr.com/noble/
  Private key de test: ec3c161597b119998ad8822acb2b1123405e85c7577e5e8abd678ad8c2383f2c
  (= echo -n luc | sha256sum)
  by Luc
*/
#ifndef REGION
#define REGION R_EU868
#endif
//#define ENABLE_LED

#define TX_INTERVAL 2 // Interval between transmissions (in seconds)

// Only use for testing
#define DEBUGSLEEP

//#include <LightTransaction.h>
#include <LibLacuna.h>
#include <SPI.h>
#include <time.h>
#include <RTC.h>

size_t protobuffer_length;

//Lora settings
static byte networkKey[] = {0x1F, 0x0D, 0xDD, 0x75, 0x0F, 0x53, 0x3D, 0xA0, 0x3B, 0xC1, 0x78, 0x39, 0x5F, 0x4C, 0x04, 0x15};
static byte appKey[] = {0xE3, 0x06, 0x9B, 0xD5, 0x11, 0x05, 0x05, 0x8A, 0x12, 0xD6, 0x9E, 0xBC, 0x09, 0xC9, 0xAD, 0xAA};
static byte deviceAddress[] = { 0x26, 0x0B, 0x56, 0x98 };
static lsLoraWANParams loraWANParams;
static lsLoraTxParams txParams;

void setup() {
  Serial.begin(9600);
  Serial3.begin(115200);
  //pinMode(LS_LED_BLUE, OUTPUT);
  //pinMode(SD_ON_OFF, OUTPUT);
  //digitalWrite(SD_ON_OFF, HIGH);

  while (!Serial && millis() < 3000);
  while (!Serial3 && millis() < 5000);

  //Serial.println("Initializing ..");

  analogReadResolution(12);

  // SX1262 configuration for lacuna LS200 board
  lsSX126xConfig cfg;
  lsCreateDefaultSX126xConfig(&cfg);

  // Initialize SX1262
  int result = lsInitSX126x(&cfg);
  // Serial.print("Init on E22/SX1262: "); Serial.println(lsErrorToString(result));

  // LoRaWAN session parameters
  lsCreateDefaultLoraWANParams(&loraWANParams, networkKey, appKey, deviceAddress);
  loraWANParams.txPort = 1;
  loraWANParams.rxEnable = false;

  // transmission parameters for terrestrial LoRa
  lsCreateDefaultLoraTxParams(&txParams);
  txParams.spreadingFactor = lsLoraSpreadingFactor_7;
  txParams.power = 16;
  txParams.preambleRelay = true;
  txParams.frequency = 865000000;
  // Serial.print("Terrestrial Uplink Frequency: ");
  //Serial.println(txParams.frequency / 1e6);
  //
  //  int result_public_key = generate_public_keys(private_key, public_key_compressed);
  //  if (!result_public_key)
  //  {
  //    Serial.print("Generate public keys failled");
  //  }
  //STM32.wdtEnable(32000);
}

void alarmMatch() {
}

void setAlarm(uint32_t alarmepoch)
{
  time_t t;
  struct tm tm;
  t = (time_t)alarmepoch;
  gmtime_r(&t, &tm);
  RTC.setAlarmTime(tm.tm_hour, tm.tm_min, tm.tm_sec);
  RTC.setAlarmDay(tm.tm_mday);
  RTC.enableAlarm(RTC.MATCH_HHMMSS);
  RTC.attachInterrupt(alarmMatch);
}

void LS200_sleep()
{
  SPI.end();
  STM32.stop();
  SPI.begin();
}

void generateMessage(void) {

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
  //STM32.wdtReset();
  Serial3.write("0\n");
  //generateMessage();
  float x, y, z, temperature, altitude, pressure;
  x = 2.5;
  y = 0.7;
  z = 2.2;
  temperature = 25.2;
  altitude = 100.5;
  pressure = 1000.5;
  byte my_protobuffer[48] = {
    ((uint8_t*)&temperature)[0],
    ((uint8_t*)&temperature)[1],
    ((uint8_t*)&temperature)[2],
    ((uint8_t*)&temperature)[3],
    ((uint8_t*)&altitude)[0],
    ((uint8_t*)&altitude)[1],
    ((uint8_t*)&altitude)[2],
    ((uint8_t*)&altitude)[3],
    ((uint8_t*)&pressure)[0],
    ((uint8_t*)&pressure)[1],
    ((uint8_t*)&pressure)[2],
    ((uint8_t*)&pressure)[3],
    ((uint8_t*)&x)[0],
    ((uint8_t*)&x)[1],
    ((uint8_t*)&x)[2],
    ((uint8_t*)&x)[3],
    ((uint8_t*)&temperature)[0],
    ((uint8_t*)&temperature)[1],
    ((uint8_t*)&temperature)[2],
    ((uint8_t*)&temperature)[3],
    ((uint8_t*)&altitude)[0],
    ((uint8_t*)&altitude)[1],
    ((uint8_t*)&altitude)[2],
    ((uint8_t*)&altitude)[3],
    ((uint8_t*)&pressure)[0],
    ((uint8_t*)&pressure)[1],
    ((uint8_t*)&pressure)[2],
    ((uint8_t*)&pressure)[3],
    ((uint8_t*)&x)[0],
    ((uint8_t*)&x)[1],
    ((uint8_t*)&x)[2],
    ((uint8_t*)&x)[3],
    ((uint8_t*)&temperature)[0],
    ((uint8_t*)&temperature)[1],
    ((uint8_t*)&temperature)[2],
    ((uint8_t*)&temperature)[3],
    ((uint8_t*)&altitude)[0],
    ((uint8_t*)&altitude)[1],
    ((uint8_t*)&altitude)[2],
    ((uint8_t*)&altitude)[3],
    ((uint8_t*)&pressure)[0],
    ((uint8_t*)&pressure)[1],
    ((uint8_t*)&pressure)[2],
    ((uint8_t*)&pressure)[3],
    ((uint8_t*)&x)[0],
    ((uint8_t*)&x)[1],
    ((uint8_t*)&x)[2],
    ((uint8_t*)&x)[3],
  
  };
  protobuffer_length = 48;
  Serial3.write("0E\n");
  Serial3.write("1\n");
  //int error = build_light_payload(private_key, public_key_compressed, my_message, my_protobuffer, &protobuffer_length);
  //   Serial.print("Mon proto est long de : "); Serial.println(protobuffer_length);
  //  Serial.print("Mon proto est : "); print_array(my_protobuffer, protobuffer_length);
  Serial3.write("1E\n");
  Serial3.write("2\n");
  int lora_result = lsSendLoraWAN(&loraWANParams, &txParams, (byte *)my_protobuffer, protobuffer_length);
  Serial3.write("2E\n");

#ifdef DEBUGSLEEP
  delay(TX_INTERVAL * 1000);
#else
  setAlarm(RTC.getEpoch() + TX_INTERVAL); //RTC.getEpoch() + TX_INTERVAL = date de vie du mc en seconde
  LS200_sleep();
#endif
}
