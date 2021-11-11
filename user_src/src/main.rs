
mod assembler {
    fn line_err(line: u32, message: &str) {
        println!("ERROR(NOT_FATAL): on line {}\r\n{}\r\n", line, message);
    }
    fn fatal_line_err(line: u32, message: &str) {
        println!("ERROR(FATAL): on line {}\r\n{}\r\n\r\nprogram will exit in 10 seconds\r\n", line, message);
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

        let file = match fs::read_to_string(file_path) {Ok(v) => v, Err(e) => {fatal_line_err(0, "there was a problem reading the file!"); "".to_string()}};

        assemble_str(file)
    }
    pub fn assemble_str(code: String) -> Vec<u32> {
        let lines = code.to_ascii_lowercase();
        let lines = lines.lines();

        let mut output = vec![0u32; 0];

        let mut line_counter = 0;

        for line in lines {
            let line = line.trim();
            let line: Vec<&str> = line.split_ascii_whitespace().collect();

            match line[..] {

                ["nop"]                 => {output.push(0x00000000)},
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
                ["read", c, "a"]        => {output.push(0x30000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},
                ["read", c, "b"]        => {output.push(0x31000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},
                ["read", c, "c"]        => {output.push(0x32000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},
                ["read", c, "d"]        => {output.push(0x33000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},
                ["wrte", "a"]           => {output.push(0x40000000)},
                ["wrte", "b"]           => {output.push(0x41000000)},
                ["wrte", "c"]           => {output.push(0x42000000)},
                ["wrte", "d"]           => {output.push(0x43000000)},
                ["add", "a", "b"]       => {output.push(0x54000000)},
                ["add", "a", "c"]       => {output.push(0x55000000)},
                ["add", "a", "d"]       => {output.push(0x56000000)},
                ["add", "b", "a"]       => {output.push(0x57000000)},
                ["add", "b", "c"]       => {output.push(0x58000000)},
                ["add", "b", "d"]       => {output.push(0x59000000)},
                ["add", "c", "a"]       => {output.push(0x5a000000)},
                ["add", "c", "b"]       => {output.push(0x5b000000)},
                ["add", "c", "d"]       => {output.push(0x5c000000)},
                ["add", "d", "a"]       => {output.push(0x5d000000)},
                ["add", "d", "b"]       => {output.push(0x5e000000)},
                ["add", "d", "c"]       => {output.push(0x5f000000)},
                ["add", c, "a"]         => {output.push(0x50000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},
                ["add", c, "b"]         => {output.push(0x51000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},
                ["add", c, "c"]         => {output.push(0x52000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},
                ["add", c, "d"]         => {output.push(0x53000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},
                ["sub", "a", "b"]       => {output.push(0x64000000)},
                ["sub", "a", "c"]       => {output.push(0x65000000)},
                ["sub", "a", "d"]       => {output.push(0x66000000)},
                ["sub", "b", "a"]       => {output.push(0x67000000)},
                ["sub", "b", "c"]       => {output.push(0x68000000)},
                ["sub", "b", "d"]       => {output.push(0x69000000)},
                ["sub", "c", "a"]       => {output.push(0x6a000000)},
                ["sub", "c", "b"]       => {output.push(0x6b000000)},
                ["sub", "c", "d"]       => {output.push(0x6c000000)},
                ["sub", "d", "a"]       => {output.push(0x6d000000)},
                ["sub", "d", "b"]       => {output.push(0x6e000000)},
                ["sub", "d", "c"]       => {output.push(0x6f000000)},
                ["sub", c, "a"]         => {output.push(0x60000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},
                ["sub", c, "b"]         => {output.push(0x61000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},
                ["sub", c, "c"]         => {output.push(0x62000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},
                ["sub", c, "d"]         => {output.push(0x63000000 & (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {fatal_line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); 0}}))},

                _ => {},
            }

            line_counter = line_counter + 1;
        };

        output
    }
}

mod emulator {
    struct Computer {
        speed_hz: f64,
        a_reg: u32,
        b_reg: u32,
        c_reg: u32,
        d_reg: u32,
        sp_reg: u32,
        ip_reg: u32,
        vl_reg: u32,
        nm_reg: u32,
        mem: [u32; 0xffffff],
        instructions: [u32; 0xffffff],
    }
    impl Computer {
        pub fn new() -> Computer {
            Computer {
                speed_hz: 0.0,
                a_reg: 0u32,
                b_reg: 0u32,
                c_reg: 0u32,
                d_reg: 0u32,
                sp_reg: 0u32,
                ip_reg: 0u32,
                vl_reg: 0u32,
                nm_reg: 0u32,
                mem: [0u32; 0xffffff],
                instructions: [0u32; 0xffffff],
            }
        }

        pub fn load_code(&mut self, code: Vec<u32>) {
            let mut counter = 0u32;
            for op in code {
                if counter < 0xffffff {
                    self.instructions[counter as usize] = op;
                } else {
                    println!("ERROR(FATAL)\r\ncode overflow! too much code! how did you get this much!?\r\n\r\nprogram will exit in 10 seconds\r\n");
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
                counter = counter + 1;
            }
        }

        pub fn run(&mut self) {
            fn err(c: &mut Computer) {println!("use of unused opcode"); panic!()};
            let mut ctrl = [err as fn(&mut Computer) -> (); 0xff];

            fn nop(c: &mut Computer) {} ctrl[0x00] = nop;
            fn mov_aa(c: &mut Computer) {c.a_reg = c.a_reg} ctrl[0x10] = mov_aa;
        }
    }
}



fn main() {
    println!("Hello, world!");
}
