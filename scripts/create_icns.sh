mkdir build_assets/osx/AppIcon.iconset
sips -z 16 16     build_assets/AppIcon.png --out build_assets/osx/AppIcon.iconset/icon_16x16.png
sips -z 32 32     build_assets/AppIcon.png --out build_assets/osx/AppIcon.iconset/icon_16x16@2x.png
sips -z 32 32     build_assets/AppIcon.png --out build_assets/osx/AppIcon.iconset/icon_32x32.png
sips -z 64 64     build_assets/AppIcon.png --out build_assets/osx/AppIcon.iconset/icon_32x32@2x.png
sips -z 128 128   build_assets/AppIcon.png --out build_assets/osx/AppIcon.iconset/icon_128x128.png
sips -z 256 256   build_assets/AppIcon.png --out build_assets/osx/AppIcon.iconset/icon_128x128@2x.png
sips -z 256 256   build_assets/AppIcon.png --out build_assets/osx/AppIcon.iconset/icon_256x256.png
sips -z 512 512   build_assets/AppIcon.png --out build_assets/osx/AppIcon.iconset/icon_256x256@2x.png
sips -z 512 512   build_assets/AppIcon.png --out build_assets/osx/AppIcon.iconset/icon_512x512.png
cp build_assets/AppIcon.png build_assets/osx/AppIcon.iconset/icon_512x512@2x.png
iconutil -c icns build_assets/osx/AppIcon.iconset
rm -R build_assets/osx/AppIcon.iconset