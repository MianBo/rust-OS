/// 藉由 sbi 来完成 set_timer
pub fn set_timer(timer: usize) {
    sbi_rt::set_timer(timer as _);
}

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{NoReason, Shutdown, SystemFailure, system_reset};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
