# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp target/$PROFILE/libtancmark.so $TAN_ROOT/@std/text/cmark/.
cp target/$PROFILE/libtanhttpclient.so $TAN_ROOT/@std/network/http/client/.
cp target/$PROFILE/libtanhttpserver.so $TAN_ROOT/@std/network/http/server/.
cp target/$PROFILE/libtanimagepng.so $TAN_ROOT/@std/image/png/.