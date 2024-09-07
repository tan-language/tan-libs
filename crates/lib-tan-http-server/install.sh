# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtanhttpserver.so $TAN_ROOT/@std/network/http/server/.
