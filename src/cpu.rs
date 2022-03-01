pub struct CPU {
    pub program_counter: u16,
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            register_a: 0,
            register_x: 0,
            status: 0,
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_z_n_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_z_n_flags(self.register_x);
    }

    fn enx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_z_n_flags(self.register_x)
    }

    /// Updates the Zero and Negative flags
    /// depending on the result passed.
    fn update_z_n_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                0x00 => {
                    return;
                }
                // LDA
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.lda(param);
                }
                // TDA
                0xAA => self.tax(),
                0xE8 => self.enx(),
                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        // assert we've loaded a 5 into the register
        assert_eq!(cpu.register_a, 0x05);
        // assert it's a non-zero number by looking at the 0 flag
        assert!(cpu.status & 0b0000_0010 == 0b00);
        // assert it's a positive number by looking at the negative flag
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_lda_load_zero() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        // assert we've loaded a 0 into the register
        assert_eq!(cpu.register_a, 0x00);
        // assert it's a zero by looking at the 0 flag
        assert!(cpu.status & 0b0000_0010 == 0b10);
        // assert it's a positive number by looking at the negative flag (0 is positive for
        // practical purposes)
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_tax_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0xaa, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.register_x, 0x05);
        // assert it's a non-zero number by looking at the 0 flag
        assert!(cpu.status & 0b0000_0010 == 0b00);
        // assert it's a positive number by looking at the negative flag
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
