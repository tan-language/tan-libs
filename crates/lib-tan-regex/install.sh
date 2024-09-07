# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtanregex.so $TAN_ROOT/@std/regex/.
