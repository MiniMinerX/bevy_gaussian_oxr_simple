## Setup
Get libopenxr_loader.so from the Oculus OpenXR Mobile SDK and add it to `runtime_libs/arm64-v8a`
https://developer.oculus.com/downloads/package/oculus-openxr-mobile-sdk/
`runtime_libs/arm64-v8a/libopenxr_loader.so`

install `xbuild`. Note that the `--git` is
very important here.
```sh
cargo install --git https://github.com/rust-mobile/xbuild
```
DID YOU INSTALL IT FROM GIT? IF NOT GO BACK AND INSTALL
IT WITH --git DO IT. DO IT NOW. IT WILL NOT WORK IF YOU HAD IT 
PREVIOUSLY INSTALLED.

published xbuild does not allow for dylibs, but the xbuild on git does.
```sh 
# List devices and copy device string "adb:***"
x devices

# Run on this device
x run --release --device adb:***
```

If you have issues with blake3 ( this is common ) build with this
```
CARGO_FEATURE_PURE=1 x run --release --device adb:***
```

This simple gaussian example runs on windows pcvr and on Quest 3 native. It uses a small gaussian splat. Larger splats are laggy on native but should be able to be run smoothly with some fixes.

Pcvr performance depends on a user by user basis.

Use using x doctor on windows, these installs are required:
- clang/llvm toolchain
    - clang, clang++,
    -llvm-...
    -lld...
- rust
    - rustup
    - cargo
- android
    - adb
    - javac
    - java
    - kotlin
    - gradle
- ios (not required for quest 3 native (android))
    - (but could be useful for a mac pcvr app)
- linux (not required for quest 3 native (android))
    - (but could be useful for a linux_pcvr app)
    - mksquashfs




KNOWN BUGS

When using x build to send the app to the quest, embedded assets 
kind of works. It will only send over the files in the assets folder, not in any subfolders of the assets folder.

IE: keep all files in the root of the assets folder for now until some wonderful soul fixes this bug. Also, and explanation of how to access quest 3 file system to put splats to hotload would be good too. Right now they have to be embedded into the exe, which requires a recompile for every new splat.