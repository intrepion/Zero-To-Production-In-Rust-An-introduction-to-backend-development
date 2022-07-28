#!/usr/bin/env bash

SCRIPT_NAME=$0
APP_ID=$1
if [ "$#" -eq 1 ]
then
    doctl app update $APP_ID --spec=spec.yaml
else
    echo "Usage: $SCRIPT_NAME <app_id>"
    exit 1
fi
