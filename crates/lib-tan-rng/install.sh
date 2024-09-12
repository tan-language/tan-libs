# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtanrng.so $TAN_ROOT/@std/rng/.
