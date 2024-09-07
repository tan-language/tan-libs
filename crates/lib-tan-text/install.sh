# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtantext.so $TAN_ROOT/@std/text/.
