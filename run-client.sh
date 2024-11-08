#!/bin/bash
set -ex
cd client
export TRUNK_SKIP_VERSION_CHECK=true
# cargo watch --why --delay 4 -s "bash -exc ' time trunk build --offline; trunk serve --offline --ignore . --ignore ..'" -w ../game/ -i ../game/target -w . -i server_data -i target -i dist  --watch ../VERSION

trunk build --offline || trunk build
trunk serve  --watch . --ignore dist --ignore target --watch ../game/ --ignore ../game/target --watch ../VERSION
