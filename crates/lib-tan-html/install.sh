# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtanhtml.so $TAN_ROOT/@std/html/.
