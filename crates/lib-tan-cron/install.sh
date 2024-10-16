# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtancron.so $TAN_ROOT/@std/cron/.
