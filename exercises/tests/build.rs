//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()//当前系统时间
        .duration_since(std::time::UNIX_EPOCH)//1970-01-01 0点到现在过去了多少秒
        .unwrap()
        .as_secs(); // What's the use of this timestamp here? //转换成整数（s）
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = format!("rustc-cfg=feature=\"pass\"");;
    println!("cargo:{}", your_command);
}
