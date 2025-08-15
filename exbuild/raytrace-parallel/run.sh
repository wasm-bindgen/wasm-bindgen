#!/bin/sh

set -ex

python3 build.py

python3 server.py
