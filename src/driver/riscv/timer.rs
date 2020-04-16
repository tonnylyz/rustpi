use riscv::regs::*;

const TIMER_DEFAULT_COUNT: usize = 250000;

pub fn next() {
  extern "C" {
    fn set_sbi_timer(n: usize, c: usize);
  }
  unsafe { set_sbi_timer(0, TIMER_DEFAULT_COUNT); }
}

pub fn init(core_id: usize) {
  //next();
  // TODO: timer not working now
}
