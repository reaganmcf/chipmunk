#!/bin/sh

echo "Building..."

wasm-pack build \
    --target no-modules \
    --out-dir ./pkg

echo "Extracting JS handlers..."

wasm-dis ./pkg/chipmunk_js_bg.wasm | grep '(import "wbg" "__wbg_CHIPMUNK' > .tmp_symbols

cat .tmp_symbols | grep -o "__wbg_CHIPMUNK[^\"]*" > .tmp_symbols2

ASYNC_FUNCS="";
while read p; do
  if [ ${#ASYNC_FUNCS} -gt 0 ] 
  then
    ASYNC_FUNCS="$ASYNC_FUNCS,asyncify-imports@wbg.$p"
  else
    ASYNC_FUNCS="asyncify-imports@wbg.$p"
  fi
done <.tmp_symbols2

ARG_LIST="--pass-arg $ASYNC_FUNCS"
echo $ARG_LIST

echo "Asyncifying JS handlers..."
wasm-opt --asyncify $ARG_LIST ./pkg/chipmunk_js_bg.wasm -o ./pkg/chipmunk_js_async.wasm

echo "Done!"
