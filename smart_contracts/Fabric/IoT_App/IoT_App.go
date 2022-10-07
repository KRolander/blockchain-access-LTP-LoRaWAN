package main

import (
	"IoT_App/chaincode"
	"log"

	"github.com/hyperledger/fabric-contract-api-go/contractapi"
)

func main() {
	IoT_AppChaincode, err := contractapi.NewChaincode(&chaincode.SmartContract{})
	if err != nil {
		log.Panicf("Error creating IoT_App chaincode: %v", err)
	}
	if err := IoT_AppChaincode.Start(); err != nil {
		log.Panicf("Error starting IoT_App chaincode: %v", err)
	}
}
