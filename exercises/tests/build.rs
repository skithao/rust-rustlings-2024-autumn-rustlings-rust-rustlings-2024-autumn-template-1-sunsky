//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::env;

fn main() {
    // 对于 tests7，设置环境变量 TEST_FOO
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // 这里我们设置 TEST_FOO 的值为当前的时间戳
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 对于 tests8，启用 pass 特性
    // 这里我们使用 cargo:rustc-cfg 指令来启用一个编译时配置
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
