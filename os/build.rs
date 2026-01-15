static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";

fn main() {
    // 告诉Cargo，如果../user/src/目录中的任何文件发生变化，就需要重新构建当前包（os内核）
    println!("cargo:rerun-if-changed=../user/src/");
    // 告诉Cargo，如果用户程序编译后的目标目录发生变化，也需要重新构建内核
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
}