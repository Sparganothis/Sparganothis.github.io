#!/bin/bash
set -ex
cd client
trunk serve --watch . --ignore dist --ignore target --watch ../game/ --ignore ../game/target