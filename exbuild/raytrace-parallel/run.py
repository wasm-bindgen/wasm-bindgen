#!/usr/bin/env python3
import os
import subprocess

root_dir = os.path.dirname(__file__)

subprocess.run(["python3", "build.py"], cwd=root_dir).check_returncode()

subprocess.run(["python3", "server.py"], cwd=root_dir).check_returncode()
