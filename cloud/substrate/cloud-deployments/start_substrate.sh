#!/bin/bash

#RUN example:
#
#./start_substrate.sh <RANCHER TOKEN>
#

nb_nodes=4

./delete-substrate-net.sh $1
sleep 10
./genNodeYaml.sh $nb_nodes > substrate-kube.yaml
./deploy-substrate.sh $1