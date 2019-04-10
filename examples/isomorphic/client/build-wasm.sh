
#!/bin/bash


cd $(dirname $0)

mkdir -p build/
mkdir -p dist/

cp static/index.html build/

if . ./bootstrap.sh; then
    wasm-pack build --target no-modules --no-typescript --out-dir ./build
fi
