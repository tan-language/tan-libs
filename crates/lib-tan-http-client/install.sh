# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtanhttpclient.so $TAN_ROOT/@std/network/http/client/.
