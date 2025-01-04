export MACOSX_DEPLOYMENT_TARGET="11"
export BINARY="redeath"
export APP_NAME="reDEATH"
export DMG_PATH="dist/${APP_NAME}-macOS-apple-silicon.dmg"

cargo build --release --target aarch64-apple-darwin
mkdir -p dist/${APP_NAME}.app/Contents/MacOS
mkdir -p dist/${APP_NAME}.app/Contents/Resources
cp target/aarch64-apple-darwin/release/${BINARY} dist/${APP_NAME}.app/Contents/MacOS
cp -r assets dist/${APP_NAME}.app/Contents/MacOS/ || true
cp build_assets/osx/Info.plist dist/${APP_NAME}.app/Contents/Info.plist
cp build_assets/osx/AppIcon.icns dist/${APP_NAME}.app/Contents/Resources/
cp build_assets/osx/installer_background.png dist/${APP_NAME}.app/Contents/Resources/

rm ${DMG_PATH}
create-dmg \
  --volname "${APP_NAME}" \
  --volicon dist/${APP_NAME}.app/Contents/Resources/AppIcon.icns \
  --background dist/${APP_NAME}.app/Contents/Resources/installer_background.png \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --icon "${APP_NAME}.app" 200 190 \
  --hide-extension "${APP_NAME}.app" \
  --app-drop-link 600 185 \
  "${DMG_PATH}" \
  "dist/${APP_NAME}.app"
