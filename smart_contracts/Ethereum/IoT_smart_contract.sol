pragma solidity >=0.4.22 <0.9.0;

import "@openzeppelin/contracts/utils/Strings.sol";


contract Sample {
    // uint8[32] data;
    bytes8[32] dataBytes;
    
    uint256 dataIoT;
    uint8 ID;
    // address public pubKeyIoT = 0xFA3cA730E9d357cc7B6D8d82E8eDF92C7c062Aec;
        
    string IndexID = "id~key";
    string IndexData = "data~key";
    string IndexCounter = "counter~key";

	string EndDevice  = "End";
	string EdgeDevice = "Edge";


    bytes32 tmpHash;  
    
    bool tmp = false;

    
   function toBytes(uint256 x) public returns (bytes memory b) {
        b = new bytes(32);
        assembly { mstore(add(b, 32), x) }
    }
  
  
    // mapping(string => bool) deviceStorage;
    mapping(address => string) public deviceGeneralStorage;
    mapping(string => bool) public deviceIDStorage;
    mapping(string => uint) public deviceCounterStorage;
    mapping(string => bytes) public deviceDataStorage;




function toAsciiString(address x) internal pure returns (string memory) {
    bytes memory s = new bytes(40);
    for (uint i = 0; i < 20; i++) {
        bytes1 b = bytes1(uint8(uint(uint160(x)) / (2**(8*(19 - i)))));
        bytes1 hi = bytes1(uint8(b) / 16);
        bytes1 lo = bytes1(uint8(b) - 16 * uint8(hi));
        s[2*i] = char(hi);
        s[2*i+1] = char(lo);            
    }
        return string(s);
    }

function char(bytes1 b) internal pure returns (bytes1 c) {
    if (uint8(b) < 10) return bytes1(uint8(b) + 0x30);
    else return bytes1(uint8(b) + 0x57);
}
   


function stringToBytes32(string memory source) public pure returns (bytes32 result) {
    bytes memory tempEmptyStringTest = bytes(source);
    if (tempEmptyStringTest.length == 0) {
        return 0x0;
    }

    assembly {
        result := mload(add(source, 32))
    }
}




function putIDsToState(string memory pubkey, string memory id) public{
     string memory compositKey = string.concat(id, pubkey);
     deviceIDStorage[compositKey] = true;
}

function putAddressesToState(address addressPubkeys, string memory pubkey) public{
    deviceGeneralStorage[addressPubkeys] = pubkey;
}



function putCounterToState(string memory pubkey, string memory id, string memory deviceType, uint counter) public{
    string memory compositKey = string.concat(id, pubkey);
    deviceCounterStorage[compositKey] = counter;
}


function initLedger() public returns (bool){
        // address[1] memory testIDs = [0xFA3cA730E9d357cc7B6D8d82E8eDF92C7c062Aec];
        string[4] memory pubKeysIoT = [
        "991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524",
		"e607a962411371b76ffc2f9f4b5d0fb78992c0f7daf853f7b660533a14f96fe4",
		"66d86f5d1da88a3df9f465a33281e2f3818117c803174f5aa7367accdcc516a6",
		"e01d8fcf7996776ebec5c23e5dae66580d49ae9981b5aa4fee324d680ad15b5f"];

        address[4] memory addressPubkeys = [
            0xeF5861B9B65614F360d8f41F989bAcc5C8761A4C,
            0x596774500dc74b9f9B057F4CEa483A9D4523C366,
            0xF6Af77e5410eCB09B21cA7b89Cb8Ca34A8A42457,
            0x968407F4a7298dB58E9758D5E8DefCA0CA475AE3
        ];

        // string memory indexStr = toAsciiString(SpubKeyIoT);
        // uint totier = pubKeysIoT.length;
        uint totier = 4;

        uint j = 0;

        for (j=0; j<totier; j++){
            putAddressesToState(addressPubkeys[j],pubKeysIoT[j]);
            // bytes memory pubKeyBytes = bytes(pubKeyIoT[j]);
            // bytes32 tmpHash = keccak256(pubKeyBytes);
            // address addressAsIndex = address(uint160(uint256(tmpHash)));
            // deviceGeneralStorage[addressAsIndex] = pubKeyIoT[j];
            
            putIDsToState(pubKeysIoT[j], IndexID);

            // string memory compositKey = string.concat(IndexID ,pubKeyIoT[j]);
            // deviceIDStorage[co // bytes memory pubKeyBytes = bytes(pubKeyIoT[j]);
            // bytes32 tmpHash = keccak256(pubKeyBytes);
            // address addressAsIndex = address(uint160(uint256(tmpHash)));
            // deviceGeneralStorage[addressAsIndex] = pubKeyIoT[j];mpositKey] = "KNOWN";

            uint firstSending = 0;
            putCounterToState(pubKeysIoT[j], IndexCounter, EndDevice,firstSending);

        }
        return deviceIDStorage["id~key991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524"];
}

function verifIDStorage(address recoveredAddress, string memory deviceType) public returns (bool){
    string memory IoTDevicePubKey = deviceGeneralStorage[recoveredAddress];
    string memory compositKey = string.concat(IndexID, IoTDevicePubKey); // TODO concatenate IndexID with deviceType
    
    return deviceIDStorage[compositKey]; // true : the device is registered, false : the device is not registered
    
}


function getCounterFromState(string memory IoTDevicePubKey, string memory deviceType) public returns (uint){
    string memory compositKey = string.concat(IndexCounter, IoTDevicePubKey);
    return deviceCounterStorage[compositKey];
}

function putDataToState(address recoveredAddress, string memory data, string memory deviceType) public returns (bool) {
    string memory IoTDevicePubKey = deviceGeneralStorage[recoveredAddress];

    uint counter = getCounterFromState(IoTDevicePubKey, deviceType);
    string memory counterStr = Strings.toString(counter);

    string memory compositKeyA = string.concat(IndexData, counterStr); 
    string memory compositKeyB = string.concat(compositKeyA, IoTDevicePubKey); // TODO concatenate IndexID with deviceType
    
    deviceDataStorage[compositKeyB] = bytes(data);


    putCounterToState(IoTDevicePubKey, IndexCounter, deviceType, counter + 1);
    
    return true;

}
function Get_Data(string memory IoTDevicePubKey, string memory deviceType, uint counter) public returns (string memory){
    
    string memory counterStr = Strings.toString(counter);
    string memory compositKeyA = string.concat(IndexData, counterStr); 
    string memory compositKeyB = string.concat(compositKeyA, IoTDevicePubKey); // TODO concatenate IndexID with deviceType   
   
    bytes memory data = deviceDataStorage[compositKeyB];
   
    string memory dataStr = string(data);

   return dataStr;

}


function Put_Data(uint256 signatureR,  uint256 signatureS, string memory data, string memory deviceType) public returns (bool, address){

    bytes32  r = bytes32(signatureR);
    bytes32  s = bytes32(signatureS);

    bytes32 hash = keccak256(bytes(data));
        
    uint8 v = 27;

    address recoveredAddress;
    address recoveredAddressB;

    recoveredAddress = ecrecover(hash, v, r, s);



    // return (verifIDStorage(recoveredAddress, deviceType), recoveredAddress);

    if(verifIDStorage(recoveredAddress, deviceType) == true)
    {
        return (putDataToState(recoveredAddress, data, deviceType), recoveredAddress);
    }
    recoveredAddressB = ecrecover(hash, v+1, r, s);
    if(verifIDStorage(recoveredAddressB, deviceType) == true)
    {
        return (putDataToState(recoveredAddress, data, deviceType), recoveredAddress);
    }
    return (false, recoveredAddress);
    
}


function queryLedger(string memory compositKey) public view returns (string memory, bool){
        // string memory compositKey = string.concat(id, pubkey);
        
        
        return (compositKey, deviceIDStorage[compositKey]);
}
    // function setMyData(uint256 _dataToStore, uint8 _IDofIoT, uint256 _r, uint256 _s) public returns (bool) {
        
    //     bytes memory fullValueToHash = new bytes (32 + 1);
    //     bytes memory valueToHash_part1 = toBytes(_dataToStore);
        
    //     bytes32  r = bytes32(_r);
    //     bytes32  s = bytes32(_s);
        
        
    //     uint i=0;
    //     for(i=0; i<32; i++)
    //     {
    //         fullValueToHash[i] = valueToHash_part1[i];
    //     }
    //     fullValueToHash[32] = byte(_IDofIoT); // valueToHash_part2
        
        
    //     bytes32 hash = keccak256(fullValueToHash);
    //     tmpHash = hash;
        
    //     uint8 v = 27;
        
    //     if ((ecrecover(hash, v, r, s) == pubKeyIoT) || (ecrecover(hash, v+1, r, s) == pubKeyIoT)) 
    //         	return true;
    //     else
    //         	return false;
        
        
    // }
    function bytes32ToString(bytes32 _bytes32) public pure returns (string memory) {
        uint8 i = 0;
        while(i < 32 && _bytes32[i] != 0) {
            i++;
        }
        bytes memory bytesArray = new bytes(i);
        for (i = 0; i < 32 && _bytes32[i] != 0; i++) {
            bytesArray[i] = _bytes32[i];
        }
        return string(bytesArray);
    }
    
    // function setMyData(uint256 _dataToStore, uint256 _r, uint256 _s) public returns (bool) {
        
    //     bytes memory fullValueToHash = new bytes (32);
    //     bytes memory valueToHash_part1 = toBytes(_dataToStore);
        
    //     bytes32  r = bytes32(_r);
    //     bytes32  s = bytes32(_s);
        
        
    //     uint i=0;
    //     for(i=0; i<32; i++)
    //     {
    //         fullValueToHash[i] = valueToHash_part1[i];
    //     }
        
        
    //     bytes32 hash = keccak256(fullValueToHash);
    //     tmpHash = hash;
        
    //     uint8 v = 27;
        
    //     if ((ecrecover(hash, v, r, s) == pubKeyIoT) || (ecrecover(hash, v+1, r, s) == pubKeyIoT)) 
    //     {
    //             tmp = true;
    //         	return true;
    //     }
    //     else{
    //             tmp = false;
    //         	return false;
    //     }
        
    // }
    

    // function my_set(uint8[32] d) public{
    //   for (uint i = 0; i < 32; i++) {
    //       data[i] = d[i];
    //   }
    // }

   

    // function my_get_bytes()  public constant returns(bytes8[32]){
    //      for (uint i = 0; i < 32; i++) {
    //         dataBytes[i] = toByte(data[i]);
    //      }
    //     return dataBytes;
    // }
    
    // function toByte(uint8 _num) returns (byte _ret) {
    //     return byte(_num);
    // }
     
  
}