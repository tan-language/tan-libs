# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtancodecjson.so $TAN_ROOT/@std/codec/json-codec/.
