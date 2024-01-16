#!/bin/sh
mkdir -p ~/Library/Application\ Support/Google/Chrome/NativeMessagingHosts/
sed -i '' "s|single_file_companion_rust.exe|$(pwd)/single_file_companion_rust|g" singlefile_companion.json
cp singlefile_companion.json ~/Library/Application\ Support/Google/Chrome/NativeMessagingHosts/