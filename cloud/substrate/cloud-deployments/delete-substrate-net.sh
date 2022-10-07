#!/bin/bash
cd ./rancher-v2.4.10/

#$1 is the rancher login token

./login.sh $1

echo "Delete Deployments"
./rancher kubectl -n substrate-net delete deployments --all

echo "Delete PVC"
./rancher kubectl -n substrate-net delete pvc --all

echo "Delete PV"

./rancher kubectl -n substrate-net get pv | awk '/substrate-/{print $1}' | xargs ./rancher kubectl -n substrate-net delete pv

echo "Delete Services"
./rancher kubectl -n substrate-net delete services --all

# ./rancher kubectl -n substrate-net delete pv --all

echo "Done"