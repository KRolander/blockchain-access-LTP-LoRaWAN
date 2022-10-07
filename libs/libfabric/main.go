package main

import "C"
import "fmt"

var fabricGlobal Fabric

//export CreateAsset
func CreateAsset(ccpPath string, credPath string, smartContract string, signature string, message string, pubkey string, deviceType string) *C.char {
	// fabric := NewFabric(ccpPath, credPath, smartContract)

	transaction, err := fabricGlobal.CreateAsset(signature, message, pubkey, deviceType)
	if err != nil {
		return C.CString(err.Error())
	}
	return C.CString(transaction)
}

//export GetAsset
func GetAsset(ccpPath string, credPath string, smartContract string, pubkey string, deviceType string, dataIndex string) *C.char {
	fabric := NewFabric(ccpPath, credPath, smartContract)
	asset, err := fabric.ReadAsset(pubkey, deviceType, dataIndex)
	if err != nil {
		fmt.Println(err)
		return nil
	}
	return C.CString(asset)
}

//export InitLedger
func InitLedger(ccpPath string, credPath string, smartContract string) {
	fabricGlobal = NewFabric(ccpPath, credPath, smartContract)

	// fabric := NewFabric(ccpPath, credPath, smartContract)
	// _, err := fabric.contract.SubmitTransaction("InitLedger")
	// if err != nil {
	// 	fmt.Println(err)
	// 	return
	// }
}

func main() {

	//ccp := "/Users/ilyagrishkov/Desktop/fabric/test-network/organizations/peerOrganizations/org1.example.com/connection-org1.yaml"
	//cred := "/Users/ilyagrishkov/Desktop/fabric/test-network/organizations/peerOrganizations/org1.example.com/users/User1@org1.example.com/msp"
	//InitLedger(ccp, cred)
}
