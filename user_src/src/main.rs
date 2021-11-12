
mod assembler {
    fn line_err(line: u32, message: &str) {
        println!("!  ERROR on line {} -> {}\r\n", line, message);
    }
    fn fatal_line_err(line: u32, message: &str) {
        println!("\r\n!! ERROR on line {}\r\n{}\r\n\r\nprogram will exit in 10 seconds\r\n", line, message);
        println!("10...");
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("5...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("4...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("3...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("2...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("1...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        panic!();
    }

    pub fn assemble_file(file_path: &str) -> Vec<u32> {
        use std::fs;

        let file = match fs::read_to_string(file_path) {Ok(v) => v, Err(_e) => {fatal_line_err(0, "there was a problem reading the file!"); "".to_string()}};

        assemble_str(file)
    }
    pub fn assemble_str(code: String) -> Vec<u32> {
        use std::collections::HashMap;
        let lower = code.to_ascii_lowercase();
        let lines_cd = lower.lines();
        let lines = lower.lines();

        let mut will_panic = false;
        let mut output = vec![0u32; 0];
        let mut line_counter = 1;

        let mut jump_map: HashMap<&str, u32> = HashMap::new();


        for line in lines {
            let line = line.trim();
            let line: Vec<&str> = line.split_ascii_whitespace().collect();

            match line[..] {
                [] => {},
                ["#label", l]           => {jump_map.insert(l, line_counter - 2); line_counter = line_counter + 1;},
                ["#", l]                => {jump_map.insert(l, line_counter - 2); line_counter = line_counter + 1;},
                _ => {line_counter = line_counter + 1;}
            }
            
        }

        line_counter = 1;

        for line in lines_cd {
            let line = line.trim();
            let line: Vec<&str> = line.split_ascii_whitespace().collect();


            match line[..] {
                ["#label", _l]           => {},
                ["#", _l]                => {},

                [".dev", "output", "a"]      => {output.push(0x01000000)},
                [".dev", "output", "b"]      => {output.push(0x02000000)},
                [".dev", "output", "c"]      => {output.push(0x03000000)},
                [".dev", "output", "d"]      => {output.push(0x04000000)},
                [".dev", "output", "g-regs"] => {output.push(0x05000000)},
                [".dev", "output", "s-regs"] => {output.push(0x06000000)},

                ["nop"]                 => {output.push(0x00000000)},
                ["hlt"]                 => {output.push(0x0f000000)},
                ["mov", "a", "a"]       => {output.push(0x10000000)},
                ["mov", "a", "b"]       => {output.push(0x11000000)},
                ["mov", "a", "c"]       => {output.push(0x12000000)},
                ["mov", "a", "d"]       => {output.push(0x13000000)},
                ["mov", "b", "a"]       => {output.push(0x14000000)},
                ["mov", "b", "b"]       => {output.push(0x15000000)},
                ["mov", "b", "c"]       => {output.push(0x16000000)},
                ["mov", "b", "d"]       => {output.push(0x17000000)},
                ["mov", "c", "a"]       => {output.push(0x18000000)},
                ["mov", "c", "b"]       => {output.push(0x19000000)},
                ["mov", "c", "c"]       => {output.push(0x1a000000)},
                ["mov", "c", "d"]       => {output.push(0x1b000000)},
                ["mov", "d", "a"]       => {output.push(0x1c000000)},
                ["mov", "d", "b"]       => {output.push(0x1d000000)},
                ["mov", "d", "c"]       => {output.push(0x1e000000)},
                ["mov", "d", "d"]       => {output.push(0x1f000000)},
                ["mov", "a", "sp"]      => {output.push(0x20000000)},
                ["mov", "a", "ip"]      => {output.push(0x21000000)},
                ["mov", "b", "sp"]      => {output.push(0x22000000)},
                ["mov", "b", "ip"]      => {output.push(0x23000000)},
                ["mov", "c", "sp"]      => {output.push(0x24000000)},
                ["mov", "c", "ip"]      => {output.push(0x25000000)},
                ["mov", "d", "sp"]      => {output.push(0x26000000)},
                ["mov", "d", "ip"]      => {output.push(0x27000000)},
                ["mov", "a", "vl"]      => {output.push(0x28000000)},
                ["mov", "a", "nm"]      => {output.push(0x29000000)},
                ["mov", "b", "vl"]      => {output.push(0x2a000000)},
                ["mov", "b", "nm"]      => {output.push(0x2b000000)},
                ["mov", "c", "vl"]      => {output.push(0x2c000000)},
                ["mov", "c", "nm"]      => {output.push(0x2d000000)},
                ["mov", "d", "vl"]      => {output.push(0x2e000000)},
                ["mov", "d", "nm"]      => {output.push(0x2f000000)},
                ["read", "a", "b"]      => {output.push(0x34000000)},
                ["read", "a", "c"]      => {output.push(0x35000000)},
                ["read", "a", "d"]      => {output.push(0x36000000)},
                ["read", "b", "a"]      => {output.push(0x37000000)},
                ["read", "b", "c"]      => {output.push(0x38000000)},
                ["read", "b", "d"]      => {output.push(0x39000000)},
                ["read", "c", "a"]      => {output.push(0x3a000000)},
                ["read", "c", "b"]      => {output.push(0x3b000000)},
                ["read", "c", "d"]      => {output.push(0x3c000000)},
                ["read", "d", "a"]      => {output.push(0x3d000000)},
                ["read", "d", "b"]      => {output.push(0x3e000000)},
                ["read", "d", "c"]      => {output.push(0x3f000000)},
                ["read", c, "a"]        => {output.push(0x30000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["read", c, "b"]        => {output.push(0x31000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["read", c, "c"]        => {output.push(0x32000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["read", c, "d"]        => {output.push(0x33000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["syswrite", "a"]       => {output.push(0x40000000)},
                ["syswrite", "b"]       => {output.push(0x41000000)},
                ["syswrite", "c"]       => {output.push(0x42000000)},
                ["syswrite", "d"]       => {output.push(0x43000000)},
                ["const", c, "a"]       => {output.push((0x44 << 24) | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the second argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["const", c, "b"]       => {output.push((0x45 << 24) | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the second argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["const", c, "c"]       => {output.push((0x46 << 24) | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the second argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["const", c, "d"]       => {output.push((0x47 << 24) | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the second argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["out", "a"]            => {output.push(0x48000000)},
                ["out", "b"]            => {output.push(0x49000000)},
                ["out", "c"]            => {output.push(0x4a000000)},
                ["out", "d"]            => {output.push(0x4b000000)},
                ["jmp", p]              => {output.push((0x50 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))}
                ["jz", p]               => {output.push((0x51 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))}
                ["jnz", p]              => {output.push((0x52 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))}
                ["jc", p]               => {output.push((0x53 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))}
                ["jnc", p]              => {output.push((0x54 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))}
                ["jzc", p]              => {output.push((0x55 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))}
                ["jzxorc", p]           => {output.push((0x56 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))}
                ["sysadd", "a", "b"]    => {output.push(0x64000000)},
                ["sysadd", "a", "c"]    => {output.push(0x65000000)},
                ["sysadd", "a", "d"]    => {output.push(0x66000000)},
                ["sysadd", "b", "a"]    => {output.push(0x67000000)},
                ["sysadd", "b", "c"]    => {output.push(0x68000000)},
                ["sysadd", "b", "d"]    => {output.push(0x69000000)},
                ["sysadd", "c", "a"]    => {output.push(0x6a000000)},
                ["sysadd", "c", "b"]    => {output.push(0x6b000000)},
                ["sysadd", "c", "d"]    => {output.push(0x6c000000)},
                ["sysadd", "d", "a"]    => {output.push(0x6d000000)},
                ["sysadd", "d", "b"]    => {output.push(0x6e000000)},
                ["sysadd", "d", "c"]    => {output.push(0x6f000000)},
                ["sysadd", c, "a"]      => {output.push(0x60000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysadd", c, "b"]      => {output.push(0x61000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysadd", c, "c"]      => {output.push(0x62000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysadd", c, "d"]      => {output.push(0x63000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["syssub", "a", "b"]    => {output.push(0x74000000)},
                ["syssub", "a", "c"]    => {output.push(0x75000000)},
                ["syssub", "a", "d"]    => {output.push(0x76000000)},
                ["syssub", "b", "a"]    => {output.push(0x77000000)},
                ["syssub", "b", "c"]    => {output.push(0x78000000)},
                ["syssub", "b", "d"]    => {output.push(0x79000000)},
                ["syssub", "c", "a"]    => {output.push(0x7a000000)},
                ["syssub", "c", "b"]    => {output.push(0x7b000000)},
                ["syssub", "c", "d"]    => {output.push(0x7c000000)},
                ["syssub", "d", "a"]    => {output.push(0x7d000000)},
                ["syssub", "d", "b"]    => {output.push(0x7e000000)},
                ["syssub", "d", "c"]    => {output.push(0x7f000000)},
                ["syssub", c, "a"]      => {output.push(0x70000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["syssub", c, "b"]      => {output.push(0x71000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["syssub", c, "c"]      => {output.push(0x72000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["syssub", c, "d"]      => {output.push(0x73000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                [] => {}
                _ => {line_err(line_counter, "not a valid operation"); will_panic = true},
            }
            line_counter = line_counter + 1;
        };

        if will_panic {
            panic!();
        }

        
        output
    }
}

mod emulator {
    pub struct Computer {
        pub speed_hz: f64,
        pub a_reg: u32,
        pub b_reg: u32,
        pub c_reg: u32,
        pub d_reg: u32,
        pub sp_reg: u32,
        pub ip_reg: u32,
        pub vl_reg: u32,
        pub nm_reg: u32,
        pub mem: [u32; 0xffffff],
        pub z_flag: bool,
        pub c_flag: bool,
        pub op_bfr: u32,
    }
    impl Computer {
        pub fn new(speed: f64) -> Computer {
            Computer {
                speed_hz: speed,
                a_reg: 0u32,
                b_reg: 0u32,
                c_reg: 0u32,
                d_reg: 0u32,
                sp_reg: 0u32,
                ip_reg: 0u32,
                vl_reg: 0u32,
                nm_reg: 0u32,
                mem: [0u32; 0xffffff],
                z_flag: false,
                c_flag: false,
                op_bfr: 0u32,
            }
        }

        pub fn run(&mut self, code: Vec<u32>) {
            fn err() {println!("use of unused opcode!"); panic!()}


            loop {

                if self.ip_reg as usize >= code.len() {break}
                self.op_bfr = code[self.ip_reg as usize];

                // println!("A: {}\r\nB: {}\r\nC: {}\r\nD: {}", self.a_reg, self.b_reg, self.c_reg, self.d_reg);
                // println!("IP:{}\r\nSP:{}\r\nVL:{}\r\nNM:{}", self.ip_reg, self.sp_reg, self.vl_reg, self.nm_reg);
                // println!("OP:{}\r\n", self.op_bfr);

                
                println!("excecuting {}", self.op_bfr);
                
                match self.op_bfr >> 24 {
                    0x01 => {println!("A register: {}", self.a_reg)}
                    0x02 => {println!("B register: {}", self.b_reg)}
                    0x03 => {println!("C register: {}", self.c_reg)}
                    0x04 => {println!("D register: {}", self.d_reg)}
                    0x05 => {println!("general pourpose registers:\r\nA:  {}\r\nB:  {}\r\nC:  {}\r\nD:  {}", self.a_reg, self.b_reg, self.c_reg, self.d_reg)}
                    0x06 => {println!("special pourpose registers:\r\nSP: {}\r\nIP: {}\r\nVL: {}\r\nNM: {}", self.sp_reg, self.ip_reg, self.vl_reg, self.nm_reg)}

                    0x00 => {},
                    0x0f => {break},
                    0x10 => {self.a_reg  = self.a_reg}
                    0x11 => {self.b_reg  = self.a_reg}
                    0x12 => {self.c_reg  = self.a_reg}
                    0x13 => {self.d_reg  = self.a_reg}
                    0x14 => {self.a_reg  = self.b_reg}
                    0x15 => {self.b_reg  = self.b_reg}
                    0x16 => {self.c_reg  = self.b_reg}
                    0x17 => {self.d_reg  = self.b_reg}
                    0x18 => {self.a_reg  = self.c_reg}
                    0x19 => {self.b_reg  = self.c_reg}
                    0x1a => {self.c_reg  = self.c_reg}
                    0x1b => {self.d_reg  = self.c_reg}
                    0x1c => {self.a_reg  = self.d_reg}
                    0x1d => {self.b_reg  = self.d_reg}
                    0x1e => {self.c_reg  = self.d_reg}
                    0x1f => {self.d_reg  = self.d_reg}
                    0x20 => {self.sp_reg = self.a_reg}
                    0x21 => {self.ip_reg = self.a_reg}
                    0x28 => {self.vl_reg = self.a_reg}
                    0x29 => {self.nm_reg = self.a_reg}
                    0x22 => {self.sp_reg = self.b_reg}
                    0x23 => {self.ip_reg = self.b_reg}
                    0x2a => {self.vl_reg = self.b_reg}
                    0x2b => {self.nm_reg = self.b_reg}
                    0x24 => {self.sp_reg = self.c_reg}
                    0x25 => {self.ip_reg = self.c_reg}
                    0x2c => {self.vl_reg = self.c_reg}
                    0x2d => {self.nm_reg = self.c_reg}
                    0x26 => {self.sp_reg = self.d_reg}
                    0x27 => {self.ip_reg = self.d_reg}
                    0x2e => {self.vl_reg = self.d_reg}
                    0x2f => {self.nm_reg = self.d_reg}
                    0x30 => {self.a_reg = self.mem[((self.op_bfr as usize) << 8) >> 8]}
                    0x31 => {self.b_reg = self.mem[((self.op_bfr as usize) << 8) >> 8]}
                    0x32 => {self.c_reg = self.mem[((self.op_bfr as usize) << 8) >> 8]}
                    0x33 => {self.d_reg = self.mem[((self.op_bfr as usize) << 8) >> 8]}
                    0x34 => {self.b_reg = self.mem[((self.a_reg as usize) << 8) >> 8]} 
                    0x35 => {self.c_reg = self.mem[((self.a_reg as usize) << 8) >> 8]} 
                    0x36 => {self.d_reg = self.mem[((self.a_reg as usize) << 8) >> 8]} 
                    0x37 => {self.a_reg = self.mem[((self.b_reg as usize) << 8) >> 8]} 
                    0x38 => {self.c_reg = self.mem[((self.b_reg as usize) << 8) >> 8]} 
                    0x39 => {self.d_reg = self.mem[((self.b_reg as usize) << 8) >> 8]} 
                    0x3a => {self.a_reg = self.mem[((self.c_reg as usize) << 8) >> 8]} 
                    0x3b => {self.b_reg = self.mem[((self.c_reg as usize) << 8) >> 8]} 
                    0x3c => {self.d_reg = self.mem[((self.c_reg as usize) << 8) >> 8]} 
                    0x3d => {self.a_reg = self.mem[((self.d_reg as usize) << 8) >> 8]} 
                    0x3e => {self.b_reg = self.mem[((self.d_reg as usize) << 8) >> 8]} 
                    0x3f => {self.c_reg = self.mem[((self.d_reg as usize) << 8) >> 8]} 
                    0x40 => {self.mem[((self.a_reg as usize) << 8) >> 8] = self.vl_reg}
                    0x41 => {self.mem[((self.b_reg as usize) << 8) >> 8] = self.vl_reg}
                    0x42 => {self.mem[((self.c_reg as usize) << 8) >> 8] = self.vl_reg}
                    0x43 => {self.mem[((self.d_reg as usize) << 8) >> 8] = self.vl_reg}
                    0x44 => {self.a_reg = ((self.op_bfr) << 8) >> 8}
                    0x45 => {self.b_reg = ((self.op_bfr) << 8) >> 8}
                    0x46 => {self.c_reg = ((self.op_bfr) << 8) >> 8}
                    0x47 => {self.d_reg = ((self.op_bfr) << 8) >> 8}
                    0x48 => {println!("{}", self.a_reg)}
                    0x49 => {println!("{}", self.b_reg)} 
                    0x4a => {println!("{}", self.c_reg)}
                    0x4b => {println!("{}", self.d_reg)}
                    0x50 => {self.ip_reg = ((self.op_bfr) << 8) >> 8}
                    0x51 => {if self.z_flag {self.ip_reg = ((self.op_bfr) << 8) >> 8}}
                    0x52 => {if !self.z_flag {self.ip_reg = ((self.op_bfr) << 8) >> 8}}
                    0x53 => {if self.c_flag {self.ip_reg = ((self.op_bfr) << 8) >> 8}}
                    0x54 => {if !self.c_flag {self.ip_reg = ((self.op_bfr) << 8) >> 8}}
                    0x55 => {if self.z_flag & self.c_flag {self.ip_reg = ((self.op_bfr) << 8) >> 8}}
                    0x56 => {if self.z_flag ^ self.c_flag {self.ip_reg = ((self.op_bfr) << 8) >> 8}}
                    0x60 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_add(self.nm_reg); self.a_reg = n.0; self.c_flag = n.1}
                    0x61 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_add(self.nm_reg); self.b_reg = n.0; self.c_flag = n.1}
                    0x62 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_add(self.nm_reg); self.c_reg = n.0; self.c_flag = n.1}
                    0x63 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_add(self.nm_reg); self.d_reg = n.0; self.c_flag = n.1}
                    0x64 => {let n = self.a_reg.overflowing_add(self.nm_reg); self.b_reg = n.0; self.c_flag = n.1}
                    0x65 => {let n = self.a_reg.overflowing_add(self.nm_reg); self.c_reg = n.0; self.c_flag = n.1}
                    0x66 => {let n = self.a_reg.overflowing_add(self.nm_reg); self.d_reg = n.0; self.c_flag = n.1}
                    0x67 => {let n = self.d_reg.overflowing_add(self.nm_reg); self.a_reg = n.0; self.c_flag = n.1}
                    0x68 => {let n = self.d_reg.overflowing_add(self.nm_reg); self.c_reg = n.0; self.c_flag = n.1}
                    0x69 => {let n = self.d_reg.overflowing_add(self.nm_reg); self.d_reg = n.0; self.c_flag = n.1}
                    0x6a => {let n = self.c_reg.overflowing_add(self.nm_reg); self.a_reg = n.0; self.c_flag = n.1}
                    0x6b => {let n = self.c_reg.overflowing_add(self.nm_reg); self.b_reg = n.0; self.c_flag = n.1}
                    0x6c => {let n = self.c_reg.overflowing_add(self.nm_reg); self.d_reg = n.0; self.c_flag = n.1}
                    0x6d => {let n = self.d_reg.overflowing_add(self.nm_reg); self.a_reg = n.0; self.c_flag = n.1}
                    0x6e => {let n = self.d_reg.overflowing_add(self.nm_reg); self.b_reg = n.0; self.c_flag = n.1}
                    0x6f => {let n = self.d_reg.overflowing_add(self.nm_reg); self.c_reg = n.0; self.c_flag = n.1}
                    0x70 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_sub(self.nm_reg); self.a_reg = n.0; self.c_flag = n.1}
                    0x71 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_sub(self.nm_reg); self.b_reg = n.0; self.c_flag = n.1}
                    0x72 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_sub(self.nm_reg); self.c_reg = n.0; self.c_flag = n.1}
                    0x73 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_sub(self.nm_reg); self.d_reg = n.0; self.c_flag = n.1}
                    0x74 => {let n = self.a_reg.overflowing_sub(self.nm_reg); self.b_reg = n.0; self.c_flag = n.1}
                    0x75 => {let n = self.a_reg.overflowing_sub(self.nm_reg); self.c_reg = n.0; self.c_flag = n.1}
                    0x76 => {let n = self.a_reg.overflowing_sub(self.nm_reg); self.d_reg = n.0; self.c_flag = n.1}
                    0x77 => {let n = self.d_reg.overflowing_sub(self.nm_reg); self.a_reg = n.0; self.c_flag = n.1}
                    0x78 => {let n = self.d_reg.overflowing_sub(self.nm_reg); self.c_reg = n.0; self.c_flag = n.1}
                    0x79 => {let n = self.d_reg.overflowing_sub(self.nm_reg); self.d_reg = n.0; self.c_flag = n.1}
                    0x7a => {let n = self.c_reg.overflowing_sub(self.nm_reg); self.a_reg = n.0; self.c_flag = n.1}
                    0x7b => {let n = self.c_reg.overflowing_sub(self.nm_reg); self.b_reg = n.0; self.c_flag = n.1}
                    0x7c => {let n = self.c_reg.overflowing_sub(self.nm_reg); self.d_reg = n.0; self.c_flag = n.1}
                    0x7d => {let n = self.d_reg.overflowing_sub(self.nm_reg); self.a_reg = n.0; self.c_flag = n.1}
                    0x7e => {let n = self.d_reg.overflowing_sub(self.nm_reg); self.b_reg = n.0; self.c_flag = n.1}
                    0x7f => {let n = self.d_reg.overflowing_sub(self.nm_reg); self.c_reg = n.0; self.c_flag = n.1}
                    _ => {err()}
                }
                self.ip_reg = self.ip_reg + 1;
                std::thread::sleep(std::time::Duration::from_secs_f64(1.0f64 / self.speed_hz));
            }

            println!("\r\nend of program")
        }
    }
}



fn main() {
    let thread_builder = std::thread::Builder::new().name("cmp".into()).stack_size(0xf000000 * 4);

    println!("mmm");
    let code = assembler::assemble_file("example.ksm");
    let handler = thread_builder.spawn(move|| {
        let mut computer = emulator::Computer::new(5.0);

        computer.run(code);
    }).unwrap();

    handler.join();

    // let computer = emulator::Computer::new(1.0);

}