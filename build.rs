// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

use std::env;

fn main() {
    // Depend on this file to prevent rebuilding on any change - see #1173 for details
    println!("cargo:rerun-if-changed=build.rs");

    // Re-export $TARGET during the build so that our executable tests know
    // what environment variable CARGO_TARGET_@TARGET@_RUNNER to check when
    // they want to spawn off executables.
    let target = env::var("TARGET").unwrap();
    println!("cargo:rustc-env=TARGET={target}");
    println!("cargo::rustc-link-arg=-fsanitize=address");
    // println!("cargo::rustc-link-arg=/Users/jevin/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/librustc-stable_rt.asan.dylib")
    println!("cargo::rustc-link-arg=/Applications/Xcode-15.4.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/15.0.0/lib/darwin/libclang_rt.asan_osx_dynamic.dylib");
    println!("cargo::rustc-link-arg=-Wl,-rpath,/Applications/Xcode-15.4.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/15.0.0/lib/darwin");

}
