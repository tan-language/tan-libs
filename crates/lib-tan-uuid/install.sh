# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtanuuid.so $TAN_ROOT/@std/uuid/.
