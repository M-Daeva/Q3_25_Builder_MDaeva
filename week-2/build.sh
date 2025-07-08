SRC_IDL_PATH="./target/idl"
SRC_TYPES_PATH="./target/types"

DIST_IDL_PATH="./scripts/common/schema/idl"
DIST_TYPES_PATH="./scripts/common/schema/types"

anchor build

rm -rf $DIST_IDL_PATH
rm -rf $DIST_TYPES_PATH

cp -r "$SRC_IDL_PATH/." $DIST_IDL_PATH
cp -r "$SRC_TYPES_PATH/." $DIST_TYPES_PATH

rm -rf $SRC_IDL_PATH
rm -rf $SRC_TYPES_PATH
