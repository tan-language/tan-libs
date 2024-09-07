# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtancodecuri.so $TAN_ROOT/@std/codec/uri/.
