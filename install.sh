# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE

#todo Make it just run the `install.sh` files in the crates?

cp target/$PROFILE/libtancmark.so $TAN_ROOT/@std/text/cmark/.
cp target/$PROFILE/libtanhttpclient.so $TAN_ROOT/@std/network/http/client/.
cp target/$PROFILE/libtanhttpserver.so $TAN_ROOT/@std/network/http/server/.
cp target/$PROFILE/libtanimagepng.so $TAN_ROOT/@std/image/png/.
cp target/$PROFILE/libtancodecuri.so $TAN_ROOT/@std/codec/uri/.
cp target/$PROFILE/libtantext.so $TAN_ROOT/@std/text/.
cp target/$PROFILE/libtanuuid.so $TAN_ROOT/@std/uuid/.
