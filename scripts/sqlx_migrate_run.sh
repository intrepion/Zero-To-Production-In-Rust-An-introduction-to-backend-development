#!/usr/bin/env bash

DATABASE_URL=$1 sqlx migrate run
