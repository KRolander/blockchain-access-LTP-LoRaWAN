package main

import (
	"crypto/x509"
	"encoding/hex"
	"encoding/pem"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"

	"github.com/hyperledger/fabric-sdk-go/pkg/core/config"
	"github.com/hyperledger/fabric-sdk-go/pkg/gateway"
)

type Fabric struct {
	PublicKey string
	contract  *gateway.Contract
}

func NewFabric(ccpPath string, credPath string, smartContract string) Fabric {
	err := os.Setenv("DISCOVERY_AS_LOCALHOST", "false")
	if err != nil {
		log.Fatalf("Error setting DISCOVERY_AS_LOCALHOST environemnt variable: %v", err)
	}

	wallet, err := gateway.NewFileSystemWallet("wallet")
	if err != nil {
		log.Fatalf("Failed to create a wallet: %v", err)
	}

	if !wallet.Exists("appUser") {
		err = populateWallet(credPath, wallet)
		if err != nil {
			log.Fatalf("Failed to populate wallet contents: %v", err)
		}
	}

	gw, err := gateway.Connect(
		gateway.WithConfig(config.FromFile(filepath.Clean(ccpPath))),
		gateway.WithIdentity(wallet, "appUser"),
		// gateway.WithTimeout(3*time.Second),
	)
	if err != nil {
		log.Fatalf("Failed to connect to gateway: %v", err)
	}

	err = os.Remove("keystore")
	if err != nil {
		log.Fatalf("Failed to remove keystore: %v", err)
	}

	defer gw.Close()

	network, err := gw.GetNetwork("mychannel")
	if err != nil {
		log.Fatalf("Failed to get network: %v", err)
	}

	identity, err := wallet.Get("appUser")
	block, _ := pem.Decode([]byte(identity.(*gateway.X509Identity).Certificate()))
	certificate, err := x509.ParseCertificate(block.Bytes)
	if err != nil {
		return Fabric{}
	}
	key, err := x509.MarshalPKIXPublicKey(certificate.PublicKey)
	if err != nil {
		return Fabric{}
	}

	return Fabric{
		PublicKey: hex.EncodeToString(key),
		contract:  network.GetContract("Iotb"),
	}
}

func (fabric Fabric) CreateAsset(signature string, message string, pubkey string, deviceType string) (string, error) {
	result, err := fabric.contract.SubmitTransaction("Put_Data", signature, message, pubkey, deviceType)
	if err != nil {
		log.Fatalf("Failed to Create asset: %v", err)
	}
	return string(result), nil
}

func (fabric Fabric) ReadAsset(pubkey string, deviceType string, dataIndex string) (string, error) {
	result, err := fabric.contract.EvaluateTransaction("Get_Data", pubkey, deviceType, dataIndex)
	if err != nil {
		return "", err
	}
	return string(result), nil
}

func populateWallet(credPath string, wallet *gateway.Wallet) error {
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
		return fmt.Errorf("keystore folder should have contain one file")
	}
	keyPath := filepath.Join(keyDir, files[0].Name())
	key, err := ioutil.ReadFile(filepath.Clean(keyPath))
	if err != nil {
		return err
	}

	identity := gateway.NewX509Identity("Org1MSP", string(cert), string(key))
	return wallet.Put("appUser", identity)
}
