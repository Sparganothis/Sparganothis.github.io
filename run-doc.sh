#!/bin/bash
set -ex
( cd client; cargo doc )
( cd game; cargo doc )
( cd server; cargo doc )