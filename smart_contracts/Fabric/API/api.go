// Author: Roland Kromes - R.G.Kromes@tudelft.nl

package main

import (
	"api/tools"
	"crypto/sha256"
	"encoding/hex"

	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"

	"github.com/decred/dcrd/dcrec/secp256k1"
	"github.com/hyperledger/fabric-sdk-go/pkg/core/config"
	"github.com/hyperledger/fabric-sdk-go/pkg/gateway"
)

// Privat Key : ec3c161597b119998ad8822acb2b1123405e857577e5e8abd678ad8c2383f20c
// Public Key : 02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524
// Recoverable Signature : 1b2a71ebe44892b8396c44640d3f2c28f7539fd62876c98737bfc96214778e646c115854627087a603f56ecd69b1712837a37838442160a67f326a9d98bee94fb5

func main() {
	fmt.Println("**** In API ****")

	var arg_ccPathConnection string
	var arg_ccPathCertificate string

	arg_ccPathConnection = os.Args[1]
	arg_ccPathCertificate = os.Args[2]
	contract, err := intiAPI(arg_ccPathConnection, arg_ccPathCertificate)
	if err != nil {
		fmt.Println("Problem wile intAPI()")
		os.Exit(1)
	}

	// // use this when the state doesn't change (get state)

	// use this when state changes (put state)

	// result, err := contract.SubmitTransaction("InitLedger")
	// if err != nil {
	// 	fmt.Printf("[InitLedger] Failed to evaluate transaction: %s\n", err)
	// 	os.Exit(1)
	// }

	// fmt.Printf("[InitLedger] Result %s \n", result)

	// result, err := contract.EvaluateTransaction("VerifIDStorage", "4f9dd78df2579279b1cbd8120808002a7d7745bd6fe666e0f368057a09ef3101", "End")

	// if err != nil {
	// 	fmt.Printf("[VerifIDStorage] Failed to evaluate transaction: %s\n", err)
	// 	os.Exit(1)
	// }

	// fmt.Printf("[VerifIDStorage] Result %s \n", result)

	var pubKeyArray []string
	var privKeyArray []string

	tools.Hello()
	///////////////////////////////////////////////////////////////////
	for i := 0; i < 64; i++ {
		PrivKey, err := secp256k1.GeneratePrivateKey()
		if err != nil {
			fmt.Println("Problem while GeneratePrivateKey()")
			os.Exit(1)
		}
		privKeyArray = append(privKeyArray, hex.EncodeToString(PrivKey.Serialize()))

		// fmt.Printf("%v\n", hex.EncodeToString(PrivKey.Serialize()))
		// hexPrivKey := hex.EncodeToString(PrivKey.Serialize())
		// pkBytes, err := hex.DecodeString(hexPrivKey)
		// privKey, _ := secp256k1.PrivKeyFromBytes(pkBytes)
		// fmt.Printf("%v\n", hex.EncodeToString(privKey.Serialize()))

		pubKey := PrivKey.PubKey()
		pubKeyArray = append(pubKeyArray, hex.EncodeToString(pubKey.SerializeCompressed()))

		// fmt.Printf("%v\n", hex.EncodeToString(pubKey.SerializeCompressed()))

		// message := "Message for ECDSA signing"
		// messageHash := sha256.Sum256([]byte(message))

		// signature, _ := secp256k1.SignCompact(PrivKey, messageHash[:], true)

		// signature[0] = signature[0] - byte(31)

		// compactSignature := signature[1:]
		// compactSignature = append(compactSignature, signature[0])

		// fmt.Printf("%v\n", hex.EncodeToString(compactSignature))

	}
	// message := "Message for ECDSA signing"

	message := "08001204496f7462180022194d65737361676520666f72204543445341207369676e696e67"

	msg, _ := hex.DecodeString(message)
	messageHash := sha256.Sum256(msg)
	// messageHash := sha256.Sum256([]byte(message))

	fmt.Println("Private Keys")
	for i := 0; i < 64; i++ {
		fmt.Printf("%v\n", privKeyArray[i])
	}

	fmt.Println("Public Keys")
	for i := 0; i < 64; i++ {
		fmt.Printf("%v\n", pubKeyArray[i])
	}
	fmt.Printf("Message : %x \n", []byte(message))

	fmt.Println("Hash : ", hex.EncodeToString(messageHash[:]))

	pkBytes, _ := hex.DecodeString("6319f9847e783c25be8e870062200fa8d73426a7f40456f3204bae52e2c41888")
	privKey, _ := secp256k1.PrivKeyFromBytes(pkBytes)
	signature, _ := secp256k1.SignCompact(privKey, messageHash[:], true)

	signature[0] = signature[0] - byte(31)

	compactSignature := signature[1:]
	compactSignature = append(compactSignature, signature[0])

	fmt.Println("Signature : ", hex.EncodeToString(compactSignature))

	result, err := contract.SubmitTransaction("Put_Data", hex.EncodeToString(compactSignature), message, "0266995b22143c71f538eaf202afa355be8e4571f15352624756a8f5e69b2d237f", "End")
	if err != nil {
		fmt.Printf("[Put_Data] Failed to evaluate transaction: %s\n", err)
		os.Exit(1)
	}
	fmt.Printf("[Put_Data] Result %s \n", result)

	///////////////////////////////////////////////////////////////////

	// hexPrivKey := "ac7460ccf2b4c49fade2ece69964d6b3060547e639a6c6d0087acc4a6b63f94d"
	// pkBytes, err := hex.DecodeString(hexPrivKey)
	// privKey, _ := secp256k1.PrivKeyFromBytes(pkBytes)
	// pubKey := privKey.PubKey()
	// hexpubKey := hex.EncodeToString(pubKey.SerializeCompressed())

	// fmt.Printf("%v\n", hexpubKey)

	// message := "Message for ECDSA signing"
	// messageHash := sha256.Sum256([]byte(message))

	// signature, _ := secp256k1.SignCompact(privKey, messageHash[:], true)

	// result, err := contract.SubmitTransaction("Put_Data", hex.EncodeToString(signature), message, hexpubKey, "End")
	// if err != nil {
	// 	fmt.Printf("[Put_Data] Failed to evaluate transaction: %s\n", err)
	// 	os.Exit(1)
	// }
	// fmt.Printf("[Put_Data] Result %s \n", result)

	// fmt.Printf(" public Key is :%v\n", hex.EncodeToString(pubKey.SerializeCompressed()))

	// Decode a hex-encoded private key.
	// pkBytes, err := hex.DecodeString("ec3c161597b119998ad8822acb2b1123405e857577e5e8abd678ad8c2383f20c")
	// if err != nil {
	// 	fmt.Println(err)
	// 	return
	// }

	// var privKeyArray []string

	// for i := 0; i < 64; i++ {
	// 	pkBytes[31] = pkBytes[31] + byte(i)

	// 	privKey, _ := secp256k1.PrivKeyFromBytes(pkBytes)

	// 	// fmt.Printf(" private Key is :%v\n", privKey)
	// 	fmt.Printf("%v\n", hex.EncodeToString(privKey.Serialize()))

	// 	privKeyArray[i] = hex.EncodeToString(privKey.Serialize())
	// 	// fmt.Printf(" public Key is :%v\n", pubKey)

	// 	// fmt.Printf(" public Key is :%v\n", hex.EncodeToString(pubKey.SerializeCompressed()))

	// }

	// // Sign a message using the private key.
	// message := "Message for ECDSA signing"
	// // messageHash := blake256.Sum256([]byte(message))
	// messageHash := sha256.Sum256([]byte(message))

	// signature := secp256k1.SignCompact(privKey.PubKey(), messageHash[:], true)

	// fmt.Printf("Serialized Signature: %x\n", signature)
	// signature := ecdsa.SignCompact(privKeyArray[0], messageHash[:], true)

	// // Serialize and display the signature.
	// fmt.Printf("Serialized Signature: %x\n", signature.Serialize())

	// // Verify the signature for the message using the public key.
	// pubKey := privKey.PubKey()

	// verified := signature.Verify(messageHash[:], pubKey)
	// fmt.Printf("Signature Verified? %v\n", verified)

	// compactSignature := ecdsa.SignCompact(privKey, messageHash[:], true)

	// fmt.Printf("Serialized Compact Signature: %x\n", compactSignature[1:])
	// fmt.Printf("Serialized Compact Signature: %x\n", compactSignature)

	// falseCompactSignature, _ := hex.DecodeString("2a71ebe44892b8396c44640d3f2c28f7539fd62876c98737bfc96214778e646c115854627087a603f56ecd69b1712837a37838442160a67f326a9d98bee94fb5")
	// i := 0
	// var iByte byte
	// found := false
	// for i = 0; i < 255; i++ {
	// 	tmpSig := make([]byte, 0)

	// 	tmpSig2 := append(tmpSig, iByte)
	// 	tmpSig3 := append(tmpSig2, falseCompactSignature...)
	// 	// fmt.Printf("Serialized Compact Signature is :%x\n", tmpSig3)

	// 	iByte++
	// 	pubKeyCompact, _, err := ecdsa.RecoverCompact(tmpSig3, messageHash[:])
	// 	if err == nil {
	// 		fmt.Printf("Compressed public Key :%x\n", pubKeyCompact.SerializeCompressed())
	// 		fmt.Printf("Compressed Signature :%x\n", tmpSig3)

	// 		found = true
	// 		break
	// 	}
	// }

	// fmt.Printf("Compressed public Key is :%v\n", found)

	// result, err = contract.EvaluateTransaction("VerifSignature", "7de4f9175ba7cebc11db681ff5e1dd43de8f3fbca3744f743223b0681b4ee60e2e9a181cd6456762a178841f2a133e8db073f8b7880ae33fe016b8fd6d975813", message, "4f9dd78df2579279b1cbd8120808002a7d7745bd6fe666e0f368057a09ef3101")

	// if err != nil {
	// 	fmt.Printf("[verifSignature] Failed to evaluate transaction: %s\n", err)
	// 	os.Exit(1)
	// }

	// fmt.Printf("[verifSignature] Result %s \n", result)

	// pubKeyCompact, _, err := ecdsa.RecoverCompact(falseCompactSignature, messageHash[:])
	// if err != nil {
	// 	fmt.Println(err)
	// 	return
	// }

	////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
	// result, err := contract.SubmitTransaction("Put_Data", "2a71ebe44892b8396c44640d3f2c28f7539fd62876c98737bfc96214778e646c115854627087a603f56ecd69b1712837a37838442160a67f326a9d98bee94fb500", message, "02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524", "End")
	// if err != nil {
	// 	fmt.Printf("[Put_Data] Failed to evaluate transaction: %s\n", err)
	// 	os.Exit(1)
	// }
	// fmt.Printf("[Put_Data] Result %s \n", result)

	// result, err := contract.SubmitTransaction("Put_Data", "2a71ebe44892b8396c44640d3f2c28f7539fd62876c98737bfc96214778e646c115854627087a603f56ecd69b1712837a37838442160a67f326a9d98bee94fb500", message, "02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524", "End")
	// if err != nil {
	// 	fmt.Printf("[Put_Data] Failed to evaluate transaction: %s\n", err)
	// 	os.Exit(1)
	// }
	// fmt.Printf("[Put_Data] Result %s \n", result)

	// result, err = contract.EvaluateTransaction("Get_Data", "02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524", "End", "0")
	// if err != nil {
	// 	fmt.Printf("[Get_Data] Failed to evaluate transaction: %s\n", err)
	// 	os.Exit(1)
	// }
	// fmt.Printf("[Get_Data] Result %s \n", result)
}

func intiAPI(ccPathConnection string, arg_ccPathCertificate string) (*gateway.Contract, error) {
	os.Setenv("DISCOVERY_AS_LOCALHOST", "true")
	wallet, err := gateway.NewFileSystemWallet("wallet")
	if err != nil {
		fmt.Printf("Failed to create wallet: %s\n", err)
		os.Exit(1)
	}

	if !wallet.Exists("appUser") {
		err = populateWallet(wallet, arg_ccPathCertificate)
		if err != nil {
			fmt.Printf("Failed to populate wallet contents: %s\n", err)
			os.Exit(1)
		}
	}

	// ccpPath := filepath.Join(
	// 	"..",
	// 	"..",
	// 	"test-network",
	// 	"organizations",
	// 	"peerOrganizations",
	// 	"org1.example.com",
	// 	"connection-org1.yaml",
	// )

	ccpPath := ccPathConnection + "/connection-org1.yaml"

	gw, err := gateway.Connect(
		gateway.WithConfig(config.FromFile(filepath.Clean(ccpPath))),
		gateway.WithIdentity(wallet, "appUser"),
	)
	if err != nil {
		fmt.Printf("Failed to connect to gateway: %s\n", err)
		os.Exit(1)
	}
	defer gw.Close()

	network, err := gw.GetNetwork("mychannel")
	if err != nil {
		fmt.Printf("Failed to get network: %s\n", err)
		os.Exit(1)
	}

	contract := network.GetContract("Iotb")
	return contract, nil
}

func populateWallet(wallet *gateway.Wallet, ccPathCertificate string) error {
	// credPath := filepath.Join(
	// 	"..",
	// 	"..",
	// 	"test-network",
	// 	"organizations",
	// 	"peerOrganizations",
	// 	"org1.example.com",
	// 	"users",
	// 	"Admin@org1.example.com",
	// 	"msp",
	// )

	credPath := ccPathCertificate

	certPath := filepath.Join(credPath, "signcerts", "cert.pem")
	// read the certificate pem
	cert, err := ioutil.ReadFile(filepath.Clean(certPath))
	if err != nil {
		return err
	}

	keyDir := filepath.Join(credPath, "keystore")
	// there's a single file in this dir containing the private key
	files, err := ioutil.ReadDir(keyDir)
	if err != nil {
		return err
	}
	if len(files) != 1 {
		return errors.New("keystore folder should have contain one file")
	}
	keyPath := filepath.Join(keyDir, files[0].Name())
	key, err := ioutil.ReadFile(filepath.Clean(keyPath))
	if err != nil {
		return err
	}

	identity := gateway.NewX509Identity("Org1MSP", string(cert), string(key))

	err = wallet.Put("appUser", identity)
	if err != nil {
		return err
	}
	return nil
}
