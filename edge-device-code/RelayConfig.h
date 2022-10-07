#ifndef __RELAY_CONFIG_H__
#define __RELAY_CONFIG_H__

/**********************************************************************************************************************************/
//#define LOWPOWER                        // Uncomment this to enable Sleep for the relay
//#define ENABLE_LED                      // On board LED enable


/* Packet filtering mode, uncomment one of two following options */
 #define FILTER_MODE_ADDRESS_ONLY       // Check address only
//#define FILTER_MODE_NETWORK_KEY_CHECK     // Check address & NwkKey

#define NUM_OF_DEVICE_IN_WHITELIST 3 // <- Set this (maximum number of device is 32)

/* Add device's info to whitelis (maximum number of device is 32) */
static byte device1_networkKey[] = {0x1F, 0x0D, 0xDD, 0x75, 0x0F, 0x53, 0x3D, 0xA0, 0x3B, 0xC1, 0x78, 0x39, 0x5F, 0x4C, 0x04, 0x15};
static byte device1_deviceAddress[] = {0x26, 0x0B, 0x56, 0x98}; 

static byte device2_networkKey[] = {0x5A, 0x9F, 0xF4, 0x6B, 0x19, 0xCC, 0x28, 0x75, 0xB4, 0x01, 0xA4, 0xF8, 0x3D, 0xBA, 0x01, 0x8C}; // Wrong NwkKey: The last 2 bytes supposed to be 0x65, 0x96
static byte device2_deviceAddress[] = {0x26, 0x0B, 0xF2, 0xDA};

static byte device3_networkKey[] = {0xC4, 0x5A, 0x1C, 0x5F, 0xD5, 0x12, 0xA7, 0xAC, 0x1C, 0xBE, 0x26, 0x5A, 0x11, 0x14, 0xB9, 0x9C};
static byte device3_deviceAddress[] = {0x26, 0x0B, 0x18, 0xCE};

// static byte device4_networkKey[] = {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
// static byte device4_deviceAddress[] = {0x00, 0x00, 0x00, 0x00};

// static byte device5_networkKey[] = {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
// static byte device5_deviceAddress[] = {0x00, 0x00, 0x00, 0x00};

// static byte device6_networkKey[] = {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
// static byte device6_deviceAddress[] = {0x00, 0x00, 0x00, 0x00};

// static byte device7_networkKey[] = {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
// static byte device7_deviceAddress[] = {0x00, 0x00, 0x00, 0x00};

// static byte device8_networkKey[] = {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
// static byte device8_deviceAddress[] = {0x00, 0x00, 0x00, 0x00};

// static byte device9_networkKey[] = {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
// static byte device9_deviceAddress[] = {0x00, 0x00, 0x00, 0x00};

// static byte device10_networkKey[] = {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
// static byte device10_deviceAddress[] = {0x00, 0x00, 0x00, 0x00};

/**********************************************************************************************************************************/
/* This section is for auto generating */
#define GET_DEVIVE_ADDRESS_UINT32(arr) (((uint32_t)arr[0] << 24) | ((uint32_t)arr[1] << 16) | ((uint32_t)arr[2] << 8) | ((uint32_t)arr[3] << 0))
#if NUM_OF_DEVICE_IN_WHITELIST > 32
#error Maximun of supported device to be relayed has been reached (32 deivce)
#endif

#if defined(FILTER_MODE_ADDRESS_ONLY) && defined(FILTER_MODE_NETWORK_KEY_CHECK)
#error Select either FILTER_MODE_ADDRESS_ONLY or FILTER_MODE_NETWORK_KEY_CHECK
#endif
#ifdef FILTER_MODE_ADDRESS_ONLY
#define RELAY_MODE lsRelayPacketCheck_AddrFilterOnly
#endif
#ifdef FILTER_MODE_NETWORK_KEY_CHECK
#define RELAY_MODE lsRelayPacketCheck_MICCheck
#endif

static uint32_t relay_device_list[NUM_OF_DEVICE_IN_WHITELIST] = {
#if NUM_OF_DEVICE_IN_WHITELIST >= 1
    GET_DEVIVE_ADDRESS_UINT32(device1_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 2
    GET_DEVIVE_ADDRESS_UINT32(device2_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 3
    GET_DEVIVE_ADDRESS_UINT32(device3_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 4
    GET_DEVIVE_ADDRESS_UINT32(device4_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 5
    GET_DEVIVE_ADDRESS_UINT32(device5_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 6
    GET_DEVIVE_ADDRESS_UINT32(device6_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 7
    GET_DEVIVE_ADDRESS_UINT32(device7_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 8
    GET_DEVIVE_ADDRESS_UINT32(device8_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 9
    GET_DEVIVE_ADDRESS_UINT32(device9_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 10
    GET_DEVIVE_ADDRESS_UINT32(device10_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 11
    GET_DEVIVE_ADDRESS_UINT32(device11_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 12
    GET_DEVIVE_ADDRESS_UINT32(device12_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 13
    GET_DEVIVE_ADDRESS_UINT32(device13_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 14
    GET_DEVIVE_ADDRESS_UINT32(device14_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 15
    GET_DEVIVE_ADDRESS_UINT32(device15_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 16
    GET_DEVIVE_ADDRESS_UINT32(device16_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 17
    GET_DEVIVE_ADDRESS_UINT32(device17_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 18
    GET_DEVIVE_ADDRESS_UINT32(device18_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 19
    GET_DEVIVE_ADDRESS_UINT32(device19_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 20
    GET_DEVIVE_ADDRESS_UINT32(device20_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 21
    GET_DEVIVE_ADDRESS_UINT32(device21_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 22
    GET_DEVIVE_ADDRESS_UINT32(device22_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 23
    GET_DEVIVE_ADDRESS_UINT32(device23_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 24
    GET_DEVIVE_ADDRESS_UINT32(device24_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 25
    GET_DEVIVE_ADDRESS_UINT32(device25_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 26
    GET_DEVIVE_ADDRESS_UINT32(device26_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 27
    GET_DEVIVE_ADDRESS_UINT32(device27_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 28
    GET_DEVIVE_ADDRESS_UINT32(device28_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 29
    GET_DEVIVE_ADDRESS_UINT32(device29_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 30
    GET_DEVIVE_ADDRESS_UINT32(device30_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 31
    GET_DEVIVE_ADDRESS_UINT32(device31_deviceAddress),
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 32
    GET_DEVIVE_ADDRESS_UINT32(device32_deviceAddress),
#endif
};

#ifdef FILTER_MODE_NETWORK_KEY_CHECK
static uint8_t *relay_nwk_key[NUM_OF_DEVICE_IN_WHITELIST] = {
#if NUM_OF_DEVICE_IN_WHITELIST >= 1
    (uint8_t *)device1_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 2
    (uint8_t *)device2_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 3
    (uint8_t *)device3_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 4
    (uint8_t *)device4_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 5
    (uint8_t *)device5_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 6
    (uint8_t *)device6_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 7
    (uint8_t *)device7_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 8
    (uint8_t *)device8_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 9
    (uint8_t *)device9_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 10
    (uint8_t *)device10_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 11
    (uint8_t *)device11_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 12
    (uint8_t *)device12_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 13
    (uint8_t *)device13_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 14
    (uint8_t *)device14_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 15
    (uint8_t *)device15_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 16
    (uint8_t *)device16_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 17
    (uint8_t *)device17_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 18
    (uint8_t *)device18_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 19
    (uint8_t *)device19_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 20
    (uint8_t *)device20_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 21
    (uint8_t *)device21_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 22
    (uint8_t *)device22_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 23
    (uint8_t *)device23_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 24
    (uint8_t *)device24_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 25
    (uint8_t *)device25_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 26
    (uint8_t *)device26_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 27
    (uint8_t *)device27_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 28
    (uint8_t *)device28_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 29
    (uint8_t *)device29_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 30
    (uint8_t *)device30_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 31
    (uint8_t *)device31_networkKey,
#endif
#if NUM_OF_DEVICE_IN_WHITELIST >= 32
    (uint8_t *)device32_networkKey,
#endif
};
#else
static uint8_t *relay_nwk_key[NUM_OF_DEVICE_IN_WHITELIST] = { NULL };
#endif

#endif /* __RELAY_CONFIG_H__ */
