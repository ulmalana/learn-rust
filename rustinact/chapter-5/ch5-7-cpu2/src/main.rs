struct CPU {
    register: [u8; 16],
    position_in_mem: usize,
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_mem;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_mem += 2;
            
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c,x,y,d) {
                (0,0,0,0) => {return;},
                (0x8, _, _, 0x4) => self.add_xy(x,y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.register[x as usize];
        let arg2 = self.register[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.register[x as usize] = val;
        if overflow {
            self.register[0xF] = 1;
        } else {
            self.register[0xF] = 0
        }
    }
}

fn main() {
    let mut cpu = CPU {
        register: [0; 16],
        memory: [0; 4096],
        position_in_mem: 0,
    };
    cpu.register[0] = 5;
    cpu.register[1] = 10;
    cpu.register[2] = 10;
    cpu.register[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80; mem[1] = 0x14;
    mem[2] = 0x80; mem[3] = 0x24;
    mem[4] = 0x80; mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.register[0], 35);
    println!("5+10+10+10 = {}", cpu.register[0]);
}
