# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtanimagepng.so $TAN_ROOT/@std/image/png/.
