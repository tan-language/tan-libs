# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtanchrono.so $TAN_ROOT/@std/chrono/.
