#!/usr/bin/env bash

doctl apps update $1 --spec spec.yaml
