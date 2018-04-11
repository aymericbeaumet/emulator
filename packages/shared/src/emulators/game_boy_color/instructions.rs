// TODO: make instruction! return a Result<Function>

macro_rules! instruction {
  {ADC $o1:ident, $o2:ident} => {
    12
  };
  {ADC $o1:ident, ($o2_ref:ident)} => {
    12
  };

  {ADD $o1:ident, $o2:ident} => {
    12
  };
  {ADD $o1:ident, ($o2_ref:ident)} => {
    12
  };

  {CCF} => {
    12
  };

  {CPL} => {
    12
  };

  {DAA} => {
    12
  };

  {DEC $o1:ident} => {
    12
  };
  {DEC ($o1:ident)} => {
    12
  };

  {HALT} => {
    12
  };

  {INC $o1:ident} => {
    12
  };
  {INC ($o1:ident)} => {
    12
  };

  {JR $o1:ident} => {
    12
  };
  {JR $o1:ident, $o2:ident} => {
    12
  };

  {LD $o1:ident, $o2:ident} => {
    12
  };
  {LD ($o1_ref:ident), $o2:ident} => {
    12
  };
  {LD $o1:ident, ($o2_ref:ident)} => {
    12
  };

  {LDD ($o1_ref:ident), $o2:ident} => {
    12
  };
  {LDD $o1:ident, ($o2_ref:ident)} => {
    12
  };

  {LDI ($o1_ref:ident), $o2:ident} => {
    12
  };
  {LDI $o1:ident, ($o2_ref:ident)} => {
    12
  };

  {NOP} => {
    4
  };

  {RLA} => {
    4
  };

  {RLCA} => {
    4
  };

  {RRA} => {
    4
  };

  {RRCA} => {
    4
  };

  {SCF} => {
    4
  };

  {STOP $o1:expr} => {
    4
  };

  {SBC $o1:ident, $o2:ident} => {
    12
  };
  {SBC $o1:ident, ($o2_ref:ident)} => {
    12
  };

  {SUB $o1:ident} => {
    12
  };
  {SUB ($o1_ref:ident)} => {
    12
  };
}

macro_rules! instruction_cb {
  () => {};
}
