# PROFILE="debug"
PROFILE="release"

cargo b --$PROFILE
cp ../../target/$PROFILE/libtancssexpr.so $TAN_ROOT/@std/dialect/css-expr/.
