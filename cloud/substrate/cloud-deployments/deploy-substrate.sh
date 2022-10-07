#!/bin/bash

cd ./rancher-v2.4.10/
./login.sh $1

echo "Load Deployments"
#big file so update config map manually using cmd line:
./rancher kubectl delete configmap chain-spec -n substrate-net
./rancher kubectl create configmap chain-spec -n substrate-net --from-file=../customSpecRaw.json

#apply main yaml:
./rancher kubectl -n substrate-net apply -f ../substrate-kube.yaml --validate=false

echo "Done"