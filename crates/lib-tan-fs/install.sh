# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtanfs.so $TAN_ROOT/@std/fs/.
