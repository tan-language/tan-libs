# PROFILE="debug"
PROFILE="release"
ROOT="/home/gmosx/root"

cargo b --$PROFILE
cp target/$PROFILE/libtancmark.so $ROOT/@std/text/cmark/.
cp target/$PROFILE/libtanhttpclient.so $ROOT/@std/network/http/client/.