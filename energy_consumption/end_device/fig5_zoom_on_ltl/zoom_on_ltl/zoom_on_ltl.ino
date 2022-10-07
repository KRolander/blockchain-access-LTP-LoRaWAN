/*
  Pour tester si le fonctionnement est ok: https://paulmillr.com/noble/
  Private key de test: ec3c161597b119998ad8822acb2b1123405e85c7577e5e8abd678ad8c2383f2c
  (= echo -n luc | sha256sum)
  by Luc
*/
#ifndef REGION
#define REGION R_EU868
#endif
#define ENABLE_LED

#define TX_INTERVAL 1 // Interval between transmissions (in seconds)

// Only use for testing
#define DEBUGSLEEP

#include <LightTransaction.h>
#include <LibLacuna.h>
#include <SPI.h>
#include <time.h>
#include <RTC.h>

byte private_key[PRIVATE_KEY_SIZE] = {0xec, 0x3c, 0x16, 0x15, 0x97, 0xb1, 0x19, 0x99, 0x8a, 0xd8, 0x82, 0x2a, 0xcb, 0x2b, 0x11, 0x23, 0x40, 0x5e, 0x85, 0xc7, 0x57, 0x7e, 0x5e, 0x8a, 0xbd, 0x67, 0x8a, 0xd8, 0xc2, 0x38, 0x3f, 0x2c};
byte public_key_compressed[PUBLIC_KEY_COMPRESSED_SIZE] = {};

Message my_message;
byte my_protobuffer[NETWORK_SIZE_MAX];
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
  while (!Serial && millis() < 3000);
  while (!Serial3 && millis() < 5000);
  analogReadResolution(12);

  // SX1262 configuration for lacuna LS200 board
  lsSX126xConfig cfg;
  lsCreateDefaultSX126xConfig(&cfg);

  // Initialize SX1262
  int result = lsInitSX126x(&cfg);

  // LoRaWAN session parameters
  lsCreateDefaultLoraWANParams(&loraWANParams, networkKey, appKey, deviceAddress);
  loraWANParams.txPort = 1;
  loraWANParams.rxEnable = false;

  // transmission parameters for terrestrial LoRa
  lsCreateDefaultLoraTxParams(&txParams);
  txParams.spreadingFactor = lsLoraSpreadingFactor_7;
  txParams.power= 16;
  txParams.preambleRelay = true;
  txParams.frequency = 865000000;

  int result_public_key = generate_public_keys(private_key, public_key_compressed);
  if (!result_public_key)
  {
   // Serial.print("Generate public keys failled");
  }
  //STM32.wdtEnable(32000);
}

void alarmMatch() {
  /*vide?*/
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

  float temperature, altitude, pressure;
  float x, y, z;

//  icm.getAGMT();
//
//  x = icm.accX();
//  y = icm.accY();
//  //z = icm.accZ();

  x = 0.1;
  y = 0.1;
  z = 0.1;

 // hp203.Measure_Sensor();

//  temperature = hp203.hp_sensorData.T;
//  altitude = hp203.hp_sensorData.A;
//  pressure = hp203.hp_sensorData.P;
//  Serial.print("Temperature : "); Serial.println(temperature);
//  Serial.print("Altitude : "); Serial.println(altitude);
//  Serial.print("Pressure: "); Serial.println(pressure);
  temperature = 29.9;
 altitude = 50.5;
 pressure = 1000.2;
  Sensors_uca_board my_sensors = {
    temperature,
    altitude,
    pressure,
    x,y,z
  };
 

  my_message = {
    Blockchain_SUBSTRATE,
    {0x69, 0x6f, 0x74, 0x62, 0x69, 0x6f, 0x74, 0x62, 0x69, 0x6f, 0x74, 0x62},
    Action_GET_DATA,
    my_sensors
  };

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

//void print_array(uint8_t * myarray, int mysize)
//{
//  char buffer[3];
//  for (int i = 0; i < mysize; i++) {
//    sprintf (buffer, "%02x", myarray[i]);
//    Serial.print(buffer);
//  }
//  Serial.println("");
//}

void loop() {

  //STM32.wdtReset();
  Serial3.write("0\n");
  generateMessage();
  Serial3.write("0E\n");
  Serial3.write("1\n");
  int error = build_light_payload(private_key, public_key_compressed, my_message, my_protobuffer, &protobuffer_length);
//Serial.print("Mon proto est long de : "); Serial.println(protobuffer_length);
//Serial.print("Mon proto est : "); print_array(my_protobuffer, protobuffer_length);
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
