package chaincode

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"strconv"

	"github.com/decred/dcrd/dcrec/secp256k1/v3/ecdsa"
	"github.com/hyperledger/fabric-contract-api-go/contractapi"
)

// SmartContract provides functions for managing an App
type SmartContract struct {
	contractapi.Contract
}

var IndexID = "id~key"
var IndexData = "data~key"
var IndexCounter = "counter~key"

type IDStructure string

const (
	EndDevice  IDStructure = "End"
	EdgeDevice IDStructure = "Edge"
)

func bussinesLogic(data string) {

}

func (s *SmartContract) InitLedger(ctx contractapi.TransactionContextInterface) error {

	testIDs := []string{"02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524",
		"02e607a962411371b76ffc2f9f4b5d0fb78992c0f7daf853f7b660533a14f96fe4",
		"0366d86f5d1da88a3df9f465a33281e2f3818117c803174f5aa7367accdcc516a6",
		"02e01d8fcf7996776ebec5c23e5dae66580d49ae9981b5aa4fee324d680ad15b5f",
	}

	for _, id := range testIDs {
		putIDsToState(ctx, EndDevice, id, "KNOWN")

		err := putCounterState(ctx, EndDevice, id, "0")
		if err != nil {
			return fmt.Errorf("failed to InitLedger : putCounterState result entry: %v", err)
		}
	}

	return nil
}

func (s *SmartContract) Get_Data(ctx contractapi.TransactionContextInterface, pubkey string, deviceType string, dataIndex string) (string, error) {

	result, err := verifIDStorage(ctx, pubkey, deviceType)
	if err != nil {
		return "failed to verifIDStorage", fmt.Errorf("failed to verifIDStorage result for %v entry: %v", result, err)
	}
	if result == "KNOWN" {
		// counter, err := getCounterState(ctx, IDStructure(deviceType), pubkey)
		// if err != nil {
		// 	return "failed to getCounterState", fmt.Errorf("failed to getCounterState result for %v entry: %v", counter, err)
		// }
		dataBytes, err := getDataState(ctx, IDStructure(deviceType), pubkey, dataIndex)
		if err != nil {
			return "failed to getDataState", fmt.Errorf("failed to getDataState result for %v entry: %v", dataBytes, err)
		}
		return string(dataBytes), nil
	} else {
		return "The signature is invalid", nil
	}

}

func (s *SmartContract) Put_Data(ctx contractapi.TransactionContextInterface, signature string, message string, pubkey string, deviceType string) (string, error) {
	// Verif if device exist
	var counterGlobal string
	result, err := verifIDStorage(ctx, pubkey, deviceType)
	if err != nil {
		return "failed to verifIDStorage", fmt.Errorf("failed to verifIDStorage result for %v entry: %v", result, err)
	}
	if result == "KNOWN" {
		resultVerif, err := verifSignature(signature, message, pubkey)
		if err != nil {
			return "failed to verifSignature", fmt.Errorf("failed to verifSignature result for %v entry: %v", resultVerif, err)
		}
		if resultVerif == "True" {
			counter, err := getCounterState(ctx, IDStructure(deviceType), pubkey)
			if err != nil {
				return "failed to getCounterState", fmt.Errorf("failed to getCounterState result for %v entry: %v", counter, err)
			}

			err = putDataToState(ctx, IDStructure(deviceType), pubkey, string(counter), message)
			if err != nil {
				return "failed to putDataToState", fmt.Errorf("failed to putDataToState result entry: %v", err)
			}
			counterGlobal = string(counter)
			counterInt, _ := strconv.Atoi(string(counter))
			counterInt++
			counterStr := strconv.Itoa(counterInt)

			err = putCounterState(ctx, IDStructure(deviceType), pubkey, counterStr)
			if err != nil {
				return "failed to putCounterState", fmt.Errorf("failed to putCounterState result entry: %v", err)
			}
		} else {
			return "The signature is invalid", nil
		}
	} else {
		return "The device is not registered to the service", nil
	}

	return "Done " + counterGlobal, nil
}

func (s *SmartContract) VerifIDStorage(ctx contractapi.TransactionContextInterface, pubkey string, deviceType string) (string, error) {
	var result string
	var err error

	if deviceType == "End" {
		result, err = verifIfExist(ctx, pubkey, EndDevice)
		if err != nil {
			return "failed to VerifIDStorage", fmt.Errorf("failed to VerifIDStorage result for %v entry: %v", result, err)
		}
	} else {
		result, err = verifIfExist(ctx, pubkey, EdgeDevice)
		if err != nil {
			return "failed to VerifIDStorage", fmt.Errorf("failed to VerifIDStorage composite result for %v entry: %v", result, err)
		}
	}

	return result, nil
}

func verifIDStorage(ctx contractapi.TransactionContextInterface, pubkey string, deviceType string) (string, error) {
	var result string
	var err error

	if deviceType == "End" {
		result, err = verifIfExist(ctx, pubkey, EndDevice)
		if err != nil {
			return "failed to VerifIDStorage", fmt.Errorf("failed to VerifIDStorage result for %v entry: %v", result, err)
		}
	} else {
		result, err = verifIfExist(ctx, pubkey, EdgeDevice)
		if err != nil {
			return "failed to VerifIDStorage", fmt.Errorf("failed to VerifIDStorage composite result for %v entry: %v", result, err)
		}
	}

	return result, nil
}

func (s *SmartContract) VerifSignature(ctx contractapi.TransactionContextInterface, signature string, message string, publicKey string) (string, error) {
	messageHash := sha256.Sum256([]byte(message))

	compactSignature, _ := hex.DecodeString(signature)
	// This part can be deleted when the sinature is 65
	i := 0
	var iByte byte
	verifiedCompact := false
	for i = 0; i < 255; i++ {
		tmpSig := make([]byte, 0)

		tmpSig2 := append(tmpSig, iByte)
		tmpSig3 := append(tmpSig2, compactSignature...)

		iByte++
		_, _, err := ecdsa.RecoverCompact(tmpSig3, messageHash[:])
		if err == nil {
			verifiedCompact = true
			break
		}
	}

	var result string
	if verifiedCompact {
		result = "True"
	} else {
		result = "False"
	}

	return result, nil
}

func verifSignature(signature string, message string, publicKey string) (string, error) {
	messageHash := sha256.Sum256([]byte(message))

	compactSignature, _ := hex.DecodeString(signature)
	// This part can be deleted when the sinature is 65
	// i := 0
	// var iByte byte
	// verifiedCompact := false
	// for i = 0; i < 255; i++ {
	// 	tmpSig := make([]byte, 0)

	// 	tmpSig2 := append(tmpSig, iByte)
	// 	tmpSig3 := append(tmpSig2, compactSignature...)

	// 	iByte++
	// 	_, _, err := ecdsa.RecoverCompact(tmpSig3, messageHash[:])
	// 	if err == nil {
	// 		verifiedCompact = true
	// 		break
	// 	}
	// }

	_, _, err := ecdsa.RecoverCompact(compactSignature, messageHash[:])
	var result string
	if err == nil {
		result = "True"
	} else {
		result = "False"
	}

	return result, nil
}

// Helper function to verify if the device is registered to the database
func verifIfExist(ctx contractapi.TransactionContextInterface, pubkey string, deviceType IDStructure) (string, error) {
	result, err := getIDsState(ctx, deviceType, pubkey)
	if string(result) != "KNOWN" {
		result = []byte("UNKNOWN")
	}
	return string(result), err
}

// pubKey is the public key of the device (hexString)
// value: KNOWN means device is registerd, UNKNOWN meins it is not registered
// For device ID (public key) storage
func putIDsToState(ctx contractapi.TransactionContextInterface, ID IDStructure, pubKey string, value string) error {
	cKey, err := ctx.GetStub().CreateCompositeKey(IndexID, []string{string(ID), pubKey})
	if err != nil {
		return fmt.Errorf("failed to create composite key for %v entry: %v", ID, err)
	}
	err = ctx.GetStub().PutState(cKey, []byte(value))
	if err != nil {
		return fmt.Errorf("failed to put state for %v entry: %v", ID, err)
	}
	return nil
}

func getIDsState(ctx contractapi.TransactionContextInterface, ID IDStructure, pubKey string) ([]byte, error) {
	cKey, err := ctx.GetStub().CreateCompositeKey(IndexID, []string{string(ID), pubKey})
	if err != nil {
		return nil, fmt.Errorf("failed to create composite key for %v key: %v", ID, err)
	}
	value, err := ctx.GetStub().GetState(cKey)
	if err != nil {
		return nil, fmt.Errorf("failed to read from world state of %v: %v", ID, err)
	}
	if value == nil {
		return nil, nil
	}

	return value, nil
}

// For data storage
func putDataToState(ctx contractapi.TransactionContextInterface, ID IDStructure, pubKey string, counter string, data string) error {
	cKey, err := ctx.GetStub().CreateCompositeKey(IndexData, []string{string(ID), pubKey, counter})
	if err != nil {
		return fmt.Errorf("putDataToState failed to create composite key for %v entry: %v", ID, err)
	}
	err = ctx.GetStub().PutState(cKey, []byte(data))
	if err != nil {
		return fmt.Errorf("putDataToState failed to put state for %v entry: %v", ID, err)
	}
	return nil
}

func getDataState(ctx contractapi.TransactionContextInterface, ID IDStructure, pubKey string, counter string) ([]byte, error) {
	cKey, err := ctx.GetStub().CreateCompositeKey(IndexData, []string{string(ID), pubKey, counter})
	if err != nil {
		return nil, fmt.Errorf("getDataState failed to create composite key for %v key: %v", ID, err)
	}
	value, err := ctx.GetStub().GetState(cKey)
	if err != nil {
		return nil, fmt.Errorf("getDataState failed to read from world state of %v: %v", ID, err)
	}
	if value == nil {
		return nil, nil
	}

	return value, nil
}

// For counter storage: counts the number of transactions sent by the device
// idnetified by its public key
func putCounterState(ctx contractapi.TransactionContextInterface, ID IDStructure, pubKey string, counter string) error {
	cKey, err := ctx.GetStub().CreateCompositeKey(IndexCounter, []string{string(ID), pubKey})
	if err != nil {
		return fmt.Errorf("putDataToState failed to create composite key for %v entry: %v", ID, err)
	}
	err = ctx.GetStub().PutState(cKey, []byte(counter))
	if err != nil {
		return fmt.Errorf("putDataToState failed to put state for %v entry: %v", ID, err)
	}
	return nil
}

func getCounterState(ctx contractapi.TransactionContextInterface, ID IDStructure, pubKey string) ([]byte, error) {
	cKey, err := ctx.GetStub().CreateCompositeKey(IndexCounter, []string{string(ID), pubKey})
	if err != nil {
		return nil, fmt.Errorf("getDataState failed to create composite key for %v key: %v", ID, err)
	}
	value, err := ctx.GetStub().GetState(cKey)
	if err != nil {
		return nil, fmt.Errorf("getDataState failed to read from world state of %v: %v", ID, err)
	}
	if value == nil {
		return nil, nil
	}

	return value, nil
}
