#!/bin/bash

eval $(minikube -p minikube docker-env)
docker-compose --env-file .env build