#!/bin/bash

TARGET_PATH='./target/thumb*/release/*'


echo "  --> [Building...] <--"
cargo b -r

echo "  --> [ObjCopy...] <--"
find $TARGET_PATH -maxdepth 0 -type f -executable -not -name '*.bin' \
  -exec arm-none-eabi-objcopy -O binary '{}' '{}.bin' \;

echo '  --> [Flushing...] <--'
find $TARGET_PATH -maxdepth 0 -type f -executable -name '*.bin' \
  -exec ./uploader/upload.sh '{}' \;


