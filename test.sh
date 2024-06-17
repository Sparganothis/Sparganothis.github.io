#!/bin/bash
set -e

echo
echo "======================================"
echo "===  TEST GAME  HOST/win32  ===="
echo "======================================"
( cd game && cargo test  )

echo
echo "======================================"
echo "===  TEST GAME  WASM/nodejs  ===="
echo "======================================"
( cd game && wasm-pack test --node )


echo
echo "======================================"
echo "===  TEST CLIENT  HOST/win32  ===="
echo "======================================"
( cd client && cargo test  )

echo
echo "======================================"
echo "===  TEST CLIENT  WASM/nodejs  ===="
echo "======================================"
( cd client && wasm-pack test --node )


echo
echo "======================================"
echo "===  TEST SEERVER HOST/win32   ===="
echo "======================================"
(  cd server && cargo test )