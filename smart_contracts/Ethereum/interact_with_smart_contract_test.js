const Web3 = require('web3')
var Tx = require('ethereumjs-tx').Transaction;

const Web3Utils = require('web3-utils');
var Eth = require('web3-eth');

const web3 = new Web3(new Web3.providers.HttpProvider('http://127.0.0.1:7545'))

const pricateKeyStr = 'deccb7a24c2c2b20f3f06bf85a807688c393e53270c265eefc1a6aa13a5396dc'
const privateKey = Buffer.from(pricateKeyStr, 'hex')

const from = '0x3BAB6793471d1BdE760D402A81F39B511f782119'

web3.eth.defaultAccount =  '0x3BAB6793471d1BdE760D402A81F39B511f782119'

const contractAddress = '0xA8da2c94CAD47180ba8d2f45892cb3d47D0EAc58' //'0x44b72e86bc2dA10BaaDFA6571c3B044dABeb0769' // 



 
const contractABI = [
	{
		"inputs": [
			{
				"internalType": "string",
				"name": "IoTDevicePubKey",
				"type": "string"
			},
			{
				"internalType": "string",
				"name": "deviceType",
				"type": "string"
			},
			{
				"internalType": "uint256",
				"name": "counter",
				"type": "uint256"
			}
		],
		"name": "Get_Data",
		"outputs": [
			{
				"internalType": "string",
				"name": "",
				"type": "string"
			}
		],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "uint256",
				"name": "signatureR",
				"type": "uint256"
			},
			{
				"internalType": "uint256",
				"name": "signatureS",
				"type": "uint256"
			},
			{
				"internalType": "string",
				"name": "data",
				"type": "string"
			},
			{
				"internalType": "string",
				"name": "deviceType",
				"type": "string"
			}
		],
		"name": "Put_Data",
		"outputs": [
			{
				"internalType": "bool",
				"name": "",
				"type": "bool"
			},
			{
				"internalType": "address",
				"name": "",
				"type": "address"
			}
		],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "bytes32",
				"name": "_bytes32",
				"type": "bytes32"
			}
		],
		"name": "bytes32ToString",
		"outputs": [
			{
				"internalType": "string",
				"name": "",
				"type": "string"
			}
		],
		"stateMutability": "pure",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "string",
				"name": "",
				"type": "string"
			}
		],
		"name": "deviceCounterStorage",
		"outputs": [
			{
				"internalType": "uint256",
				"name": "",
				"type": "uint256"
			}
		],
		"stateMutability": "view",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "string",
				"name": "",
				"type": "string"
			}
		],
		"name": "deviceDataStorage",
		"outputs": [
			{
				"internalType": "bytes",
				"name": "",
				"type": "bytes"
			}
		],
		"stateMutability": "view",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "address",
				"name": "",
				"type": "address"
			}
		],
		"name": "deviceGeneralStorage",
		"outputs": [
			{
				"internalType": "string",
				"name": "",
				"type": "string"
			}
		],
		"stateMutability": "view",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "string",
				"name": "",
				"type": "string"
			}
		],
		"name": "deviceIDStorage",
		"outputs": [
			{
				"internalType": "bool",
				"name": "",
				"type": "bool"
			}
		],
		"stateMutability": "view",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "string",
				"name": "IoTDevicePubKey",
				"type": "string"
			},
			{
				"internalType": "string",
				"name": "deviceType",
				"type": "string"
			}
		],
		"name": "getCounterFromState",
		"outputs": [
			{
				"internalType": "uint256",
				"name": "",
				"type": "uint256"
			}
		],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [],
		"name": "initLedger",
		"outputs": [
			{
				"internalType": "bool",
				"name": "",
				"type": "bool"
			}
		],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "address",
				"name": "addressPubkeys",
				"type": "address"
			},
			{
				"internalType": "string",
				"name": "pubkey",
				"type": "string"
			}
		],
		"name": "putAddressesToState",
		"outputs": [],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "string",
				"name": "pubkey",
				"type": "string"
			},
			{
				"internalType": "string",
				"name": "id",
				"type": "string"
			},
			{
				"internalType": "string",
				"name": "deviceType",
				"type": "string"
			},
			{
				"internalType": "uint256",
				"name": "counter",
				"type": "uint256"
			}
		],
		"name": "putCounterToState",
		"outputs": [],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "address",
				"name": "recoveredAddress",
				"type": "address"
			},
			{
				"internalType": "string",
				"name": "data",
				"type": "string"
			},
			{
				"internalType": "string",
				"name": "deviceType",
				"type": "string"
			}
		],
		"name": "putDataToState",
		"outputs": [
			{
				"internalType": "bool",
				"name": "",
				"type": "bool"
			}
		],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "string",
				"name": "pubkey",
				"type": "string"
			},
			{
				"internalType": "string",
				"name": "id",
				"type": "string"
			}
		],
		"name": "putIDsToState",
		"outputs": [],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "string",
				"name": "compositKey",
				"type": "string"
			}
		],
		"name": "queryLedger",
		"outputs": [
			{
				"internalType": "string",
				"name": "",
				"type": "string"
			},
			{
				"internalType": "bool",
				"name": "",
				"type": "bool"
			}
		],
		"stateMutability": "view",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "string",
				"name": "source",
				"type": "string"
			}
		],
		"name": "stringToBytes32",
		"outputs": [
			{
				"internalType": "bytes32",
				"name": "result",
				"type": "bytes32"
			}
		],
		"stateMutability": "pure",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "uint256",
				"name": "x",
				"type": "uint256"
			}
		],
		"name": "toBytes",
		"outputs": [
			{
				"internalType": "bytes",
				"name": "b",
				"type": "bytes"
			}
		],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "address",
				"name": "recoveredAddress",
				"type": "address"
			},
			{
				"internalType": "string",
				"name": "deviceType",
				"type": "string"
			}
		],
		"name": "verifIDStorage",
		"outputs": [
			{
				"internalType": "bool",
				"name": "",
				"type": "bool"
			}
		],
		"stateMutability": "nonpayable",
		"type": "function"
	}
]
const publicKeyToAddress = require('ethereum-public-key-to-address')

console.log(publicKeyToAddress('0x02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524')) // '0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1'
console.log(publicKeyToAddress('0x02e607a962411371b76ffc2f9f4b5d0fb78992c0f7daf853f7b660533a14f96fe4'))
console.log(publicKeyToAddress('0x0366d86f5d1da88a3df9f465a33281e2f3818117c803174f5aa7367accdcc516a6'))
console.log(publicKeyToAddress('0x02e01d8fcf7996776ebec5c23e5dae66580d49ae9981b5aa4fee324d680ad15b5f'))

const ethereumV = new Eth()


const ethmessage = "\x19Ethereum Signed Message:\n" + "Message for ECDSA signing".length + "Message for ECDSA signing"
messageHex = web3.utils.asciiToHex("Message for ECDSA signing")
// messageHex = web3.utils.asciiToHex(ethmessage)

message = ethereumV.abi.encodeParameter('bytes', messageHex)
// console.log(message)

const messageHash = web3.utils.sha3('Message for ECDSA signing');
console.log(messageHash)
const createKeccakHash = require('keccak')
const hash = createKeccakHash('keccak256').update(ethmessage).digest()
console.log(hash)
// const signature = await web3.eth.personal.sign(messageHash, web3.eth.defaultAccount);
const signature =  web3.eth.accounts.sign('Message for ECDSA signing', 'ec3c161597b119998ad8822acb2b1123405e857577e5e8abd678ad8c2383f20c') 
console.log(signature)


sigR = ethereumV.abi.encodeParameter('uint256', signature.r)
sigS = ethereumV.abi.encodeParameter('uint256', signature.s)

const addressObj = web3.eth.accounts.recover(signature);
console.log(addressObj)

// const crypto = require('crypto')
// const assert = require('assert')
// const Secp256k1 = require('@enumatech/secp256k1-js')

// const privatekey = Secp256k1.uint256("ec3c161597b119998ad8822acb2b1123405e857577e5e8abd678ad8c2383f20c", 16)
// const digestUint256 = Secp256k1.uint256("011a775441ecb14943130a16f00cdd41818a83dd04372f3259e3ca7237e3cdaa",16)

// // const digest = web3.utils.keccak256("Message for ECDSA signing") 
// // const digest =  web3.utils.soliditySha3('Message for ECDSA signing%')
// // const digestUint256 = Secp256k1.uint256("7cad0ffcff806f85052a581ec30d313a5293877fd02a0fee54dd68690eacbfbb", 16)

// const sig = Secp256k1.ecsign(privatekey, digestUint256)
// console.log(sig)

// const sigR = Secp256k1.uint256(signature.r,16)
// const sigS = Secp256k1.uint256(signature.s,16)



// encodedParam2 = ethereumV.abi.encodeParameters(['uint256','uint256', 'bytes'],['0xdb8a042224c44b05a97e5f2a410ea604d818bbe9e6a5d2beed5778e79efd4acf', '0xe9014ba4b484ee12141af45cdbd9641e2a053d4f69a103132fb440d27f572695', '0x0dbc83e447221890cbc667ce36f098bfd1bf1df83870236fd35d5317209a83e9']) // data, ID, r, s
// console.log(encodedParam2)
// 0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1

const contract =  new web3.eth.Contract(contractABI,contractAddress);
// web3.eth.getTransactionCount(web3.eth.defaultAccount, (err, txCount) => {

// 	const txObject = {
// 	  nonce:    web3.utils.toHex(txCount),
// 	  gasLimit: web3.utils.toHex(800000), // Raise the gas limit to a much higher amount
// 	  gasPrice: web3.utils.toHex(web3.utils.toWei('10', 'gwei')),
// 	  to: contractAddress,
// 	  data: contract.methods.Put_Data(sigR, sigS, ethmessage, "End").encodeABI()
// 	//   data: contract.methods.initLedger().encodeABI()
// 	}
	
// 	const tx = new Tx(txObject)
// 	tx.sign(privateKey)
  
// 	const serializedTx = tx.serialize()
// 	const raw = '0x' + serializedTx.toString('hex')
	
// 	var tmpHash;
// 	web3.eth.sendSignedTransaction(raw, (err, txHash) => {
// 		tmpHash = txHash
// 	  console.log('err:', err, 'txHash:', txHash)
// 	  // Use this txHash to find the contract on Etherscan!
// 	})
// 	// web3.eth.getTransactionReceipt('0xeecd72f5ede269e6c864d9e36142a865bebeb6c0f51cdd2c53b7293193a00265', (err, txReceipt)=>{
// 	// 	console.log(err,txReceipt);
// 	//    })
//   })

  console.log("Call of Put_Data \n");
// const abiEncoded1 = contract.methods.queryLedger("id~key991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524").call((err, result) => { console.log(result) })
// const abiEncoded1 = contract.methods.Put_Data(sigR, sigS, ethmessage, "End").call((err, result) => { console.log(result) })

const abiEncoded1 = contract.methods.Get_Data("991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524", "End", 2).call((err, result) => { console.log(result) })

// 	const txObject = {
// 	  nonce:    web3.utils.toHex(txCount),
// 	  gasLimit: web3.utils.toHex(800000), // Raise the gas limit to a much higher amount
// 	  gasPrice: web3.utils.toHex(web3.utils.toWei('10', 'gwei')),
// 	  to: contractAddress,
// 	  data: contract.methods.Put_Data(sigR, sigS, ethmessage, "End").encodeABI()
// 	}
	
// 	const tx = new Tx(txObject)
// 	tx.sign(privateKey)
  
// 	const serializedTx = tx.serialize()
// 	const raw = '0x' + serializedTx.toString('hex')
	
// 	var tmpHash;
// 	web3.eth.sendSignedTransaction(raw, (err, txHash) => {
// 		tmpHash = txHash
// 	  console.log('err:', err, 'txHash:', txHash)
// 	  // Use this txHash to find the contract on Etherscan!
// 	})
// 	// web3.eth.getTransactionReceipt('0xbb4d06d7783e7179aa374454923a0be9356f4c494d94deff004eedb9eaa4a2dc', (err, txReceipt)=>{
// 	// 	console.log(err,txReceipt);
// 	//    })
//   })





// // const tab = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31] 
// const tab  = [0xdb, 0x8a, 0x04, 0x22, 0x24, 0xc4, 0x4b, 0x05, 0xa9, 0x7e, 0x5f, 0x2a, 0x41, 0x0e, 0xa6, 0x04, 0xd8, 0x18, 0xbb, 0xe9, 0xe6, 0xa5, 0xd2, 0xbe, 0xed, 0x57, 0x78, 0xe7, 0x9e, 0xfd, 0x4a, 0xcf]

// const encodedFonction = web3.sha3('setMyData(uint256,uint8,uint256,uint256)').substring(0, 10)
// console.log('encode function name', encodedFonction)


// // //////////////////////////////////////////////////////////////////////////////////////////
// const ethereumV = new Eth()
// // const encodedParam = ethereumV.abi.encodeParameter('uint', 1)
// // encodedParam = ethereumV.abi.encodeFunctionSignature('get()')
// // console.log('Encoded Parameters ', encodedParam)
// //5ef329400224904f644b77896a0844a6d94e4840534ef8070ca5a3e4c321cccf

// encodedParam2 = ethereumV.abi.encodeParameters(['uint256','uint8','uint256','uint256'],['0xdb8a042224c44b05a97e5f2a410ea604d818bbe9e6a5d2beed5778e79efd4acf', '0x77', '0xe9014ba4b484ee12141af45cdbd9641e2a053d4f69a103132fb440d27f572695', '0x0dbc83e447221890cbc667ce36f098bfd1bf1df83870236fd35d5317209a83e9']) // data, ID, r, s
// console.log('Encoded Parameters2 ', encodedParam2)

// const SmartContractPayload = encodedFonction + encodedParam2.substring(2, encodedParam2.lenght)
// console.log('Smart Contract Payload', SmartContractPayload)



// const txCount = web3.eth.getTransactionCount(from)
// console.log('txCount : ', txCount)



// const txObject = {
//     nonce: txCount,
//     from: from,
//     gasPrice: '0x4A817C800', //'0x4A817C801',
//     gas: '0x6691B7', 
//     to: contractAddress,
//     // value: '0xde0b6b3a7640000',
           
//     data: SmartContractPayload
// }
 
// const tx = new Tx(txObject)
// tx.sign(privateKey)


// const serializedTx = tx.serialize()
// const raw = '0x' + serializedTx.toString('hex')
// console.log('raw : ',raw)



// web3.eth.sendRawTransaction(raw, (err, txHash) => {
//     console.log('txHash:', txHash)
// })

// //////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Call if the tab was filled 

// var dappTestContract = web3.eth.contract(contractABI, contractAddress)


// var dappTestContract = new web3.eth.Contract(contractABI, contractAddress);


// const contract =  new web3.eth.Contract(contractABI,contractAddress);

// const abiEncoded = contract.methods.initLedger().send(function(error, result) {console.log('result ' + result, 'error ' + error)})
// const abiEncoded = contract.methods.initLedger().send({from : '0xD9dB99D768862D908A325EE69f2692DED3Fc8cCC', gas: '1000000'}, function(error, result){console.log('result ' + result, 'error ' + error)})
// const abiEncoded1 = contract.methods.queryLedger("id~key991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524").send({from : '0xD9dB99D768862D908A325EE69f2692DED3Fc8cCC', gas: '1000000'}, function(error, result){console.log('result ' + result, 'error ' + error)})


// const abiEncoded1 = contract.methods.queryLedger("id~key991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524").call((err, result) => { console.log(result) })


// Use THIS : 
// const abiEncoded = dappTestContract.my_get(function(error, result) {console.log('result ' + result, 'error ' + error)})
// const abiEncoded = dappTestContract.my_set(2) //.call(function(error, result) {console.log('result ' + result, 'error ' + error)})
// const abiEncoded = dappTestContract.my_set(tab) //.call(function(error, result) {console.log('result ' + result, 'error ' + error)})
 
// const abiEncoded = dappTestContract.my_get_bytes(function(error, result) {console.log('result ' + result, 'error ' + error)})
// const abiEncoded = dappTestContract.my_get(function(error, result) {console.log('result ' + result, 'error ' + error)})

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


// const txFromHash = web3.eth.getTransaction('0x53ff67652294acc938290bea7893614e2eedbfc64dee29c00f00b42ee82cb1d7')
// console.log('Transaction from hash ', txFromHash)

// const tx_data = '0xe6523c0616ce4c0e401f0078b554017c6863587602630e1b130a376ef15c98d0' //txFromHash.inputs;
// console.log('tx_data ', tx_data)



// const input_data = '0x' + tx_data.substring(0, 10)

// const tx_params = ethereumV.abi.decodeParameter('bool', input_data)
// console.log(' Results ', tx_params)





// const txObject = {
//     nonce: '0x0',
//     gasPrice: '0x4A817C800',
//     gasLimit: '0x6691B7',
//     // to: '0xC78E793b56b7ab3CC7B4ec4dB3471343585a06Ad',
//     // value: '0xde0b6b3a7640000',
//     data: '0x6060604052341561000f57600080fd5b61010d8061001e6000396000f3006060604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b114604e5780636d4ce63c14606e575b600080fd5b3415605857600080fd5b606c60048080359060200190919050506098565b005b3415607857600080fd5b607e60a2565b604051808215151515815260200191505060405180910390f35b8060008190555050565b600060405180807f4f4b000000000000000000000000000000000000000000000000000000000000815250600201905060405180910390a060019050905600a165627a7a723058209362bee176273fa9aa78e2593f1e8d3de4f5cd1409567f43c67b662a1a3039130029'
// }

// web3.eth.getBalance('0x21133d23E04e58E846c2fA45fD5F2b75Cf91B204', (err, wei) => { balance = web3.fromWei(wei, 'ether'); console.log(balance.toString())})
// const jsw = '0xf1018504a817c800836691b794c78e793b56b7ab3cc7b4ec4db3471343585a06ad880de0b6b3a7640000846d4ce63c1c8080'            
                
// const cpp = '0xf1018504a817c800836691b794c78e793b56b7ab3cc7b4ec4db3471343585a06ad88de0b6b3a76400000846d4ce63c1c8080'



//  ################# Sign and Send Transaction #####################
// const tx = new Tx(txObject)
// tx.sign(privateKey)

// const serializedTx = tx.serialize()
// const raw = '0x' + serializedTx.toString('hex')
// console.log('raw : ',raw)

// web3.eth.sendRawTransaction(raw, (err, txHash) => {
//     console.log('txHash:', txHash)
// })

// var Tx = require('ethereumjs-tx');
// var privateKey = new Buffer('e331b6d69882b4cb4ea581d88e0b604039a3de5967688d3dcffdd2270c0fd109', 'hex')

// var rawTx = {
//   nonce: '0x00',
//   gasPrice: '0x09184e72a000',
//   gasLimit: '0x2710',
//   to: '0x0000000000000000000000000000000000000000',
//   value: '0x00',
//   data: '0x7f7465737432000000000000000000000000000000000000000000000000000000600057'
// }
// 09 18 4e 72 a0 00


// var tx = new Tx(rawTx);
// tx.sign(privateKey);

// var serializedTx = tx.serialize();

// // console.log(serializedTx.toString('hex'));
// // 0xf889808609184e72a00082271094000000000000000000000000000000000000000080a47f74657374320000000000000000000000000000000000000000000000000000006000571ca08a8bbf888cfa37bbf0bb965423625641fc956967b81d12e23709cead01446075a01ce999b56a8a88504be365442ea61239198e23d1fce7d00fcfc5cd3b44b7215f

// web3.eth.sendSignedTransaction('0x' + serializedTx.toString('hex'))
// .on('receipt', console.log);

// > // see eth.getTransactionReceipt() for details
