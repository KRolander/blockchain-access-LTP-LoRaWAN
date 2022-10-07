/*
  https://paulmillr.com/noble/
  Private key de test: ec3c161597b119998ad8822acb2b1123405e857577e5e8abd678ad8c2383f20c
  by Luc
*/

#ifndef REGION
#define REGION R_EU868
#endif


#define TX_INTERVAL 10 // Interval between transmissions (in seconds)

// Only use for testing
//#define DEBUGSLEEP

#include <LightTransaction.h>
#include <LibLacuna.h>
#include <SPI.h>
#include <time.h>
#include <RTC.h>
#include <ICM_20948.h>
#include <HP203B.h>

//LTL
byte private_key[PRIVATE_KEY_SIZE] = {0xec , 0x3c , 0x16 , 0x15 , 0x97 , 0xb1 , 0x19 , 0x99 , 0x8a , 0xd8 , 0x82 , 0x2a , 0xcb , 0x2b , 0x11 , 0x23 , 0x40 , 0x5e , 0x85 , 0x75 , 0x77 , 0xe5 , 0xe8 , 0xab , 0xd6 , 0x78 , 0xad , 0x8c , 0x23 , 0x83 , 0xf2 , 0x0c};
byte public_key_compressed[PUBLIC_KEY_COMPRESSED_SIZE] = {};
Message my_message;
byte my_protobuffer[NETWORK_SIZE_MAX];
size_t protobuffer_length;


// Set Lora settings
static byte networkKey[] = {0x1F, 0x0D, 0xDD, 0x75, 0x0F, 0x53, 0x3D, 0xA0, 0x3B, 0xC1, 0x78, 0x39, 0x5F, 0x4C, 0x04, 0x15};
static byte appKey[] = {0xE3, 0x06, 0x9B, 0xD5, 0x11, 0x05, 0x05, 0x8A, 0x12, 0xD6, 0x9E, 0xBC, 0x09, 0xC9, 0xAD, 0xAA};
static byte deviceAddress[] = { 0x26, 0x0B, 0x56, 0x98 };
static lsLoraWANParams loraWANParams;
static lsLoraTxParams txParams;

// Sensors: HP203B & ICM
ICM_20948_I2C icm;
HP203B hp203;

void setup() {
  Serial.begin(9600);
  pinMode(SD_ON_OFF, OUTPUT); //Pin for using sensors
  digitalWrite(SD_ON_OFF, HIGH);

  while (!Serial && millis() < 3000);
  Serial.println("Initializing ..");

  // Initialize I2C used by IMU
  Wire.begin();

  // Initialize IMU
  icm.begin(Wire, 0);

  // Set sample rate to ~20Hz
  icm.setSampleRate(ICM_20948_Internal_Acc , {56}); // a = 56 -> 20.09Hz, g = 55 -> 20Hz

  hp203.getAddr_HP203B(0x77);
  hp203.setOSR(OSR_4096);
  hp203.begin();
  analogReadResolution(12);

  // SX1262 configuration for lacuna LS200 board
  lsSX126xConfig cfg;
  lsCreateDefaultSX126xConfig(&cfg);

  // Initialize SX1262
  int result = lsInitSX126x(&cfg);
  Serial.print("Init on E22/SX1262: "); Serial.println(lsErrorToString(result));

  // LoRaWAN session parameters
  lsCreateDefaultLoraWANParams(&loraWANParams, networkKey, appKey, deviceAddress);
  loraWANParams.txPort = 1;
  loraWANParams.rxEnable = false;

  // transmission parameters for terrestrial LoRa
  lsCreateDefaultLoraTxParams(&txParams);
  txParams.spreadingFactor = lsLoraSpreadingFactor_7;
  txParams.preambleRelay = true;
  txParams.frequency = 865000000;
  txParams.power = 16;
  Serial.print("Terrestrial Uplink Frequency: ");
  Serial.println(txParams.frequency / 1e6);

  int result_public_key = generate_public_keys(private_key, public_key_compressed);
  if (!result_public_key)
  {
   // Serial.print("Generate public keys failled");
  }
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
  digitalWrite(SD_ON_OFF, LOW); //Sensor supply pin off
  STM32.stop();
  SPI.begin();
  digitalWrite(SD_ON_OFF, HIGH); //Sensor supply pin on
  delay(2); //Useful for starting up the sensors (with ultra low power ON)
}

void generateMessage(void) {

  float temperature, altitude, pressure;
  float x, y, z;

  icm.getAGMT();

  x = icm.accX();
  y = icm.accY();
  z = icm.accZ();

  hp203.Measure_Sensor();

  temperature = hp203.hp_sensorData.T;
  altitude = hp203.hp_sensorData.A;
  pressure = hp203.hp_sensorData.P;

  //  Serial.print("Temperature : "); Serial.println(temperature);
  //  Serial.print("Altitude : "); Serial.println(altitude);
  //  Serial.print("Pressure: "); Serial.println(pressure);

  Sensors_uca_board my_sensors = {
    temperature,
    altitude,
    pressure,
    x, y, z
  };

  my_message = {
    Blockchain_SUBSTRATE,
    {0x69, 0x6f, 0x74, 0x62}, //"iotb"
    Action_GET_DATA,
    my_sensors
  };
}

void loop() {
  generateMessage();
  int LTL_result = build_light_payload(private_key, public_key_compressed, my_message, my_protobuffer, &protobuffer_length);
  //Serial.print("Mon proto est long de : "); Serial.println(protobuffer_length);
  //Serial.print("Mon proto est : "); print_array(my_protobuffer, protobuffer_length);
  //Serial.print("Result LTL: "); Serial.println(printif_error(LTL_result));

  int lora_result = lsSendLoraWAN(&loraWANParams, &txParams, (byte *)my_protobuffer, protobuffer_length);
  // Serial.print("Result LoRa: "); Serial.println(lsErrorToString(lora_result)); Serial.println("");

#ifdef DEBUGSLEEP
  delay(TX_INTERVAL * 1000);
#else
  setAlarm(RTC.getEpoch() + TX_INTERVAL); //RTC.getEpoch() + TX_INTERVAL = date de vie du mc en seconde
  LS200_sleep();
#endif
}
