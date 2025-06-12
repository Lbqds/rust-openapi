#!/bin/bash

VERSION=v3.15.2
curl https://raw.githubusercontent.com/alephium/alephium/${VERSION}/api/src/main/resources/openapi.json -o openapi.json
openapi-generator generate -i ./openapi.json -g rust -o ./
rm openapi.json
rm git_push.sh
