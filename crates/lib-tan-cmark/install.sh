# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtancmark.so $TAN_ROOT/@std/text/cmark/.
