#!/bin/bash
my_dir="$(dirname "$0")"

NBNODES=$1
DOCKER_IMAGE_NAME="projetsim/substrate-iot-blockhain:latest"
#list of validators available
#Max 6 validator for the moment
#array built with generateKeys.sh script
# accountArray=('alice' 'bob' 'charlie' 'dave' 'eve' 'ferdie')

#include the keys file:
chmod +x $my_dir/keys_file.sh
source $my_dir/keys_file.sh

cat << EOF
apiVersion: v1
kind: List

items:

EOF

for (( i=0; i<=$NBNODES; i++ ))
do
   echo ""
   echo "# --------------------------=== POD DEPLOYMENT $i ===--------------------------"

    if [[ "$i" -eq 0 ]]; then
    #first node is bootnode

###################### set all in keystore in bootnode
cmd_add_to_keystore=""
for (( j=1; j<=$NBNODES; j++ )) # start 1 => no bootnode
do
cmd_add_to_keystore+=$(cat <<EOF

                    node-template key insert --base-path /datas/substrate-$i --chain local --key-type aura --suri "${Ed25519_arr_secretSeed[j]}";
                    node-template key insert --base-path /datas/substrate-$i --chain local --key-type gran --suri "${Ed25519_arr_secretSeed[j]}";

EOF
)

done
###################### end set all in keystore in bootnode


cat << EOF
- apiVersion: apps/v1
  kind: Deployment
  metadata:
    name: node-$i
    namespace: substrate-net
  spec:
    replicas: 1
    selector:
      matchLabels:
        name: substrate-$i
    template:
      metadata:
        labels:
          name: substrate-$i
          # serviceSelector: substrate-node
      spec:
        securityContext:
          fsGroup: 101
        containers:
          - name: substrate-node
            image: $DOCKER_IMAGE_NAME
            resources:
              requests:
                memory: "10Gi"
                cpu: "4"
                ephemeral-storage: "1500Mi"
              limits:
                memory: "11Gi"
                cpu: "4"
                ephemeral-storage: "2Gi"
            ports:
              - name: p2p
                containerPort: 30333
              - name: websocket
                containerPort: 9944
              - name: rpc
                containerPort: 9933
              - name: prometheus
                containerPort: 9615
            command:
              - bash
            args:
              - -c
              - |
                    rm -rf /datas/substrate-$i/*;
                    node-template key insert \\
                        --base-path /datas/substrate-$i \\
                        --chain local \\
                        --key-type aura \\
                        --scheme Sr25519 \\
                        --suri "0x0000000000000000000000000000000000000000000000000000000000000001";
                    node-template key insert \\
                        --base-path /datas/substrate-$i \\
                        --chain local \\
                        --key-type gran \\
                        --scheme Ed25519 \\
                        --suri "0x0000000000000000000000000000000000000000000000000000000000000001";
                    ls -l /datas/substrate-$i/chains/local_testnet/keystore;
                    # Start Alice's node
                    RUST_LOG=runtime=debug
                    node-template \\
                        --base-path /datas/substrate-$i \\
                        --name Node$i \\
                        --chain /genesis/customSpecRaw.json \\
                        --port 30333 \\
                        --ws-port 9944 \\
                        --rpc-port 9933 \\
                        --node-key 0000000000000000000000000000000000000000000000000000000000000001 \\
                        --unsafe-ws-external \\
                        --unsafe-rpc-external \\
                        --rpc-cors=all \\
                        --prometheus-external \\
                        --log info \\
                        --wasm-execution Compiled \\
                        --ws-max-connections 1000 \\
                        --pool-limit 10000 \\
                        --pool-kbytes 125000 \\
                        --validator \\
                        --state-cache-size 2147483648 \\
                        --max-runtime-instances 100
                    
            volumeMounts:
              - name: substrate-data-$i
                mountPath: /datas/substrate-$i
              - name: substrate-genesis-$i
                mountPath: /genesis/

        volumes:
          - name: substrate-data-$i
            persistentVolumeClaim:
              claimName: substrate-data-$i
          - name: substrate-genesis-$i
            configMap:
              name: chain-spec
              items:
              - key: customSpecRaw.json
                path: customSpecRaw.json
EOF

    else
    #than we have all other nodes

cat << EOF
- apiVersion: apps/v1
  kind: Deployment
  metadata:
    name: node-$i
    namespace: substrate-net
  spec:
    replicas: 1
    selector:
      matchLabels:
        name: substrate-$i
    template:
      metadata:
        labels:
          name: substrate-$i
          serviceSelector: substrate-node
      spec:
        securityContext:
          fsGroup: 101
        containers:
          - name: substrate-node
            image: $DOCKER_IMAGE_NAME
            resources:
              requests:
                memory: "10Gi"
                cpu: "4"
                ephemeral-storage: "1500Mi"
              limits:
                memory: "11Gi"
                cpu: "4"
                ephemeral-storage: "2Gi"
            ports:
              - name: p2p
                containerPort: 30333
              - name: websocket
                containerPort: 9944
              - name: rpc
                containerPort: 9933
              - name: prometheus
                containerPort: 9615
            command:
              - bash
            args:
              - -c
              - |
                    rm -rf /datas/substrate-$i/*;
                    node-template key insert \\
                        --base-path /datas/substrate-$i \\
                        --chain local \\
                        --key-type aura \\
                        --scheme Sr25519 \\
                        --suri "${Sr25519_arr_secretSeed[i]}";
                    node-template key insert \\
                        --base-path /datas/substrate-$i \\
                        --chain local \\
                        --key-type gran \\
                        --scheme Ed25519 \\
                        --suri "${Ed25519_arr_secretSeed[i]}";
                    ls -l /datas/substrate-$i/chains/local_testnet/keystore;
                    RUST_LOG=runtime=debug
                    node-template \\
                        --base-path /datas/substrate-$i \\
                        --name Node$i \\
                        --chain /genesis/customSpecRaw.json \\
                        --keystore-path /datas/substrate-$i/chains/local_testnet/keystore/ \\
                        --node-key ${Ed25519_arr_secretSeed[i]:2:64} \\
                        --port 30333 \\
                        --ws-port 9944 \\
                        --rpc-port 9933 \\
                        --unsafe-ws-external \\
                        --unsafe-rpc-external \\
                        --rpc-cors=all \\
                        --prometheus-external \\
                        --log info \\
                        --wasm-execution Compiled \\
                        --ws-max-connections 1000 \\
                        --pool-limit 10000 \\
                        --pool-kbytes 125000 \\
                        --max-runtime-instances 100 \\
                        --state-cache-size 2147483648 \\
                        --validator \\
                        --bootnodes /ip4/\$SUBSTRATE_0_SERVICE_HOST/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
                    
            volumeMounts:
              - name: substrate-data-$i
                mountPath: /datas/substrate-$i
              - name: substrate-genesis-$i
                mountPath: /genesis/

        volumes:
          - name: substrate-data-$i
            persistentVolumeClaim:
              claimName: substrate-data-$i
          - name: substrate-genesis-$i
            configMap:
              name: chain-spec
              items:
              - key: customSpecRaw.json
                path: customSpecRaw.json
EOF

fi # end if

# define service for node
cat << EOF

#---------------------------------=NODES SERVICES $i=---------------------------------------
- apiVersion: v1
  kind: Service
  metadata:
    name: substrate-$i
    namespace: substrate-net
  spec:
    type: ClusterIP
    selector:
      name: substrate-$i
    ports:
      - name: "30333"
        protocol: TCP
        port: 30333
        targetPort: 30333
      - name: "9944"
        protocol: TCP
        port: 9944
        targetPort: 9944
      - name: "9933"
        protocol: TCP
        port: 9933
        targetPort: 9933
      - name: "9615"
        protocol: TCP
        port: 9615
        targetPort: 9615
EOF

# define volume for node
cat << EOF
#---------------------------------=NODES PERSISTANT VOLUME $i=---------------------------------------
- apiVersion: v1
  kind: PersistentVolume
  metadata:
    name: substrate-$i
    labels:
      type: local
  spec:
    storageClassName: manual
    capacity:
      storage: 50Gi
    accessModes:
      - ReadWriteOnce
    persistentVolumeReclaimPolicy: Recycle
    hostPath:
      path: "/datas/substrate-$i"
EOF

# define volume claim for node
cat << EOF
#--------------------------=PERSISTENT VOLUME CLAIM $i=------------------------------

- apiVersion: v1
  kind: PersistentVolumeClaim
  metadata:
    labels:
      app: substrate-data
    name: substrate-data-$i
    namespace: substrate-net
  spec:
    storageClassName: manual
    accessModes:
    - ReadWriteOnce
    resources:
     requests:
        storage: 45Gi
EOF


done # end for loop

cat << EOF

#--------------------------=ONE SERVICE FOR ALL NODE (websocket)=--------------------------------

- apiVersion: v1
  kind: Service
  metadata:
    name: substrate-ws-service
    namespace: substrate-net
  spec:
    type: ClusterIP
    selector:
      serviceSelector: substrate-node
    ports:
      - name: "9944"
        protocol: TCP
        port: 9944
        targetPort: 9944
EOF

################################## chain spec build #################################

#first get chain spec
docker pull $DOCKER_IMAGE_NAME > out.log 2> err.log
docker run -it $DOCKER_IMAGE_NAME sh -c "node-template build-spec --disable-default-bootnode --chain local --disable-log-color 2>/dev/null" > customSpec.json
chainSpec=$(cat customSpec.json | sed 1d) #get file content and remove first line (has some unwanted output)
# chainSpec=$(cat customSpec.json) #get file content and remove first line (has some unwanted output)
echo $chainSpec > customSpec.json #write file content

###################### make palletAura authorities (Sr25519 keys)
palletAura_authorities="["
for (( i=1; i<=$NBNODES; i++ )) # start 1 => no bootnode
do
palletAura_authorities+=$(cat <<EOF
    "${Sr25519_arr_ss58PublicKey[i]}",

EOF
)

done
palletAura_authorities=${palletAura_authorities::-1} #DON'T FORGET TO REMOVE LAST CHARACTER: ${palletAura_authorities::-1}
palletAura_authorities+="]"
palletAura_authorities=$(echo "$palletAura_authorities" | jq -c) #format json to a one line
###################### end make palletAura authorities

###################### make palletGrandpa authorities (Ed25519 keys)
palletGrandpa_authorities="["
for (( i=1; i<=$NBNODES; i++ )) # start 1 => no bootnode
do
palletGrandpa_authorities+=$(cat <<EOF
    [
    "${Ed25519_arr_ss58PublicKey[i]}",
    1
    ],

EOF
)

done
palletGrandpa_authorities=${palletGrandpa_authorities::-1} #DON'T FORGET TO REMOVE LAST CHARACTER: ${palletGrandpa_authorities::-1}
palletGrandpa_authorities+="]"
palletGrandpa_authorities=$(echo "$palletGrandpa_authorities" | jq -c) #format json to a one line
###################### end make palletAura authorities



#edit json to replace the two arrays
#jq
chainSpec=$(echo $chainSpec | jq ".genesis.runtime.aura.authorities = ${palletAura_authorities}")
chainSpec=$(echo $chainSpec | jq ".genesis.runtime.grandpa.authorities = ${palletGrandpa_authorities}")
chainSpec=$(echo $chainSpec | jq '.name = "The Batman Chain"')
chainSpec=$(echo $chainSpec | jq '.id = "TBC_testnet"')

echo $chainSpec | jq > customSpec.json #write changes to file

#build raw chainSpec
docker run -it -v $(pwd)/customSpec.json:/customSpec.json $DOCKER_IMAGE_NAME  sh -c "node-template build-spec --chain=/customSpec.json --raw --disable-default-bootnode --disable-log-color 2>/dev/null" > customSpecRaw.json
chainSpecRaw=$(cat customSpecRaw.json | sed 1d) #get file content and remove first line (has some unwanted output)
# chainSpecRaw=$(cat customSpecRaw.json) 
#finish up json formating to a one line
# echo $chainSpecRaw | jq | sed 's/^/      /' > customSpecRaw.json #write changes to file and add indentation
echo $chainSpecRaw | jq > customSpecRaw.json #write changes to file and add indentation
chainSpecRaw=$(cat customSpecRaw.json) #write changes to file




# cat << EOF
# ####################################### BENCHMARK MACHINE #########################

# - apiVersion: apps/v1
#   kind: Deployment
#   metadata:
#     name: benchmark
#     namespace: substrate-net
#   spec:
#     replicas: 1
#     selector:
#         matchLabels:
#           name: benchmark-deployment
#     template:
#       metadata:
#         labels:
#           name: benchmark-deployment
#           serviceSelector: benchmark-deployment
#       spec:
#         hostAliases:
#         - ip: "185.52.32.4"
#           hostnames:
#           - "substrate-ws.unice.cust.tasfrance.com"
#         containers:
#         - name: substrate-sim-transaction-js
#           image: projetsim/substrate-sim-transaction-js:latest
#           command:
#             - "sleep"
#             - "604800"
#           resources:
#             limits:
#               cpu: "30"
#               memory: "10Gi"
#             requests:
#               cpu: "30"
#               memory: "10Gi"
#           imagePullPolicy: Always
#         restartPolicy: Always
# EOF