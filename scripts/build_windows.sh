export TARGET="x86_64-pc-windows-gnu"
export BINARY="redeath"
export APP_NAME="reDEATH"

cargo build --release --target ${TARGET}
rm -rf dist/${APP_NAME}.windows
mkdir -p dist/${APP_NAME}.windows
cp target/${TARGET}/release/${BINARY}.exe dist/${APP_NAME}.windows
cp -r assets dist/${APP_NAME}.windows || true
# TODO: Figure out why this doesn't compress as much as doing through the UI...
# zip -r dist/${APP_NAME}.windows.zip dist/${APP_NAME}.windows
