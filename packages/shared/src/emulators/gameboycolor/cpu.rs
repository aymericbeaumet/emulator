use std::time::Duration;
use std::time::Instant;
use std::thread::sleep;

pub struct Cpu {
  cycle_duration_nanos: u64,
  register_af: u16,
  register_bc: u16,
  register_de: u16,
  register_hl: u16,
  register_sp: u16,
  register_pc: u16,
}

impl Cpu {
  pub fn new(frequency: f64) -> Self {
    Cpu {
      cycle_duration_nanos: (1_000_000_000. / frequency) as u64,
      register_af: 0x01B0,
      register_bc: 0x0013,
      register_de: 0x00D8,
      register_hl: 0x014D,
      register_sp: 0xFFFE,
      register_pc: 0,
    }
  }

  pub fn start(&mut self) {
    loop {
      let time = Instant::now();
      // do some CPU work
      let elapsed = time.elapsed();
      let elapsed_nanos = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;
      if elapsed_nanos < self.cycle_duration_nanos {
        sleep(Duration::from_nanos(
          self.cycle_duration_nanos - elapsed_nanos,
        ))
      }
    }
  }
}
