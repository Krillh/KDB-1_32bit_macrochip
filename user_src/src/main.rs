
mod assembler {
    fn line_err(line: u32, message: &str) {
        println!("!  ERROR on line {} -> {}", line, message);
    }
    fn fatal_line_err(line: u32, message: &str) {
        println!("\r\n!! ERROR on line {}\r\n{}\r\n\r\npress enter to exit program\r\n", line, message);
        let mut _bffr = String::new();
        std::io::stdin().read_line(&mut _bffr).unwrap();
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
                ["jmp", p]              => {output.push((0x50 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))},
                ["jz", p]               => {output.push((0x51 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))},
                ["jnz", p]              => {output.push((0x52 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))},
                ["jc", p]               => {output.push((0x53 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))},
                ["jnc", p]              => {output.push((0x54 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))},
                ["jzc", p]              => {output.push((0x55 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))},
                ["jzxorc", p]           => {output.push((0x56 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))},
                ["call", p]             => {output.push((0x57 << 24) | (match jump_map.get(p) {Some(v) => v, None => {line_err(line_counter, "use of unassigned label"); will_panic = true; &0}}))},
                ["ret"]                 => {output.push(0x58000000)},
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
                ["sysand", "a", "b"]    => {output.push(0x84000000)},
                ["sysand", "a", "c"]    => {output.push(0x85000000)},
                ["sysand", "a", "d"]    => {output.push(0x86000000)},
                ["sysand", "b", "a"]    => {output.push(0x87000000)},
                ["sysand", "b", "c"]    => {output.push(0x88000000)},
                ["sysand", "b", "d"]    => {output.push(0x89000000)},
                ["sysand", "c", "a"]    => {output.push(0x8a000000)},
                ["sysand", "c", "b"]    => {output.push(0x8b000000)},
                ["sysand", "c", "d"]    => {output.push(0x8c000000)},
                ["sysand", "d", "a"]    => {output.push(0x8d000000)},
                ["sysand", "d", "b"]    => {output.push(0x8e000000)},
                ["sysand", "d", "c"]    => {output.push(0x8f000000)},
                ["sysand", c, "a"]      => {output.push(0x80000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysand", c, "b"]      => {output.push(0x81000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysand", c, "c"]      => {output.push(0x82000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysand", c, "d"]      => {output.push(0x83000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysor", "a", "b"]     => {output.push(0x94000000)},
                ["sysor", "a", "c"]     => {output.push(0x95000000)},
                ["sysor", "a", "d"]     => {output.push(0x96000000)},
                ["sysor", "b", "a"]     => {output.push(0x97000000)},
                ["sysor", "b", "c"]     => {output.push(0x98000000)},
                ["sysor", "b", "d"]     => {output.push(0x99000000)},
                ["sysor", "c", "a"]     => {output.push(0x9a000000)},
                ["sysor", "c", "b"]     => {output.push(0x9b000000)},
                ["sysor", "c", "d"]     => {output.push(0x9c000000)},
                ["sysor", "d", "a"]     => {output.push(0x9d000000)},
                ["sysor", "d", "b"]     => {output.push(0x9e000000)},
                ["sysor", "d", "c"]     => {output.push(0x9f000000)},
                ["sysor", c, "a"]       => {output.push(0x90000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysor", c, "b"]       => {output.push(0x91000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysor", c, "c"]       => {output.push(0x92000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysor", c, "d"]       => {output.push(0x93000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysxor", "a", "b"]    => {output.push(0xa4000000)},
                ["sysxor", "a", "c"]    => {output.push(0xa5000000)},
                ["sysxor", "a", "d"]    => {output.push(0xa6000000)},
                ["sysxor", "b", "a"]    => {output.push(0xa7000000)},
                ["sysxor", "b", "c"]    => {output.push(0xa8000000)},
                ["sysxor", "b", "d"]    => {output.push(0xa9000000)},
                ["sysxor", "c", "a"]    => {output.push(0xaa000000)},
                ["sysxor", "c", "b"]    => {output.push(0xab000000)},
                ["sysxor", "c", "d"]    => {output.push(0xac000000)},
                ["sysxor", "d", "a"]    => {output.push(0xad000000)},
                ["sysxor", "d", "b"]    => {output.push(0xae000000)},
                ["sysxor", "d", "c"]    => {output.push(0xaf000000)},
                ["sysxor", c, "a"]      => {output.push(0xa0000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysxor", c, "b"]      => {output.push(0xa1000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysxor", c, "c"]      => {output.push(0xa2000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysxor", c, "d"]      => {output.push(0xa3000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysnot", "a", "b"]    => {output.push(0xb4000000)},
                ["sysnot", "a", "c"]    => {output.push(0xb5000000)},
                ["sysnot", "a", "d"]    => {output.push(0xb6000000)},
                ["sysnot", "b", "a"]    => {output.push(0xb7000000)},
                ["sysnot", "b", "c"]    => {output.push(0xb8000000)},
                ["sysnot", "b", "d"]    => {output.push(0xb9000000)},
                ["sysnot", "c", "a"]    => {output.push(0xba000000)},
                ["sysnot", "c", "b"]    => {output.push(0xbb000000)},
                ["sysnot", "c", "d"]    => {output.push(0xbc000000)},
                ["sysnot", "d", "a"]    => {output.push(0xbd000000)},
                ["sysnot", "d", "b"]    => {output.push(0xbe000000)},
                ["sysnot", "d", "c"]    => {output.push(0xbf000000)},
                ["sysnot", c, "a"]      => {output.push(0xb0000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysnot", c, "b"]      => {output.push(0xb1000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysnot", c, "c"]      => {output.push(0xb2000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysnot", c, "d"]      => {output.push(0xb3000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
                ["sysinc", "a", "b"]    => {output.push(0xc4000000)},
                ["sysinc", "a", "c"]    => {output.push(0xc5000000)},
                ["sysinc", "a", "d"]    => {output.push(0xc6000000)},
                ["sysinc", "b", "a"]    => {output.push(0xc7000000)},
                ["sysinc", "b", "c"]    => {output.push(0xc8000000)},
                ["sysinc", "b", "d"]    => {output.push(0xc9000000)},
                ["sysinc", "c", "a"]    => {output.push(0xca000000)},
                ["sysinc", "c", "b"]    => {output.push(0xcb000000)},
                ["sysinc", "c", "d"]    => {output.push(0xcc000000)},
                ["sysinc", "d", "a"]    => {output.push(0xcd000000)},
                ["sysinc", "d", "b"]    => {output.push(0xce000000)},
                ["sysinc", "d", "c"]    => {output.push(0xcf000000)},
                ["sysinc", "sp", "a"]   => {output.push(0xc0000000)},
                ["sysinc", "sp", "b"]   => {output.push(0xc1000000)},
                ["sysinc", "sp", "c"]   => {output.push(0xc2000000)},
                ["sysinc", "sp", "d"]   => {output.push(0xc3000000)},
                ["sysdec", "a", "b"]    => {output.push(0xd4000000)},
                ["sysdec", "a", "c"]    => {output.push(0xd5000000)},
                ["sysdec", "a", "d"]    => {output.push(0xd6000000)},
                ["sysdec", "b", "a"]    => {output.push(0xd7000000)},
                ["sysdec", "b", "c"]    => {output.push(0xd8000000)},
                ["sysdec", "b", "d"]    => {output.push(0xd9000000)},
                ["sysdec", "c", "a"]    => {output.push(0xda000000)},
                ["sysdec", "c", "b"]    => {output.push(0xdb000000)},
                ["sysdec", "c", "d"]    => {output.push(0xdc000000)},
                ["sysdec", "d", "a"]    => {output.push(0xdd000000)},
                ["sysdec", "d", "b"]    => {output.push(0xde000000)},
                ["sysdec", "d", "c"]    => {output.push(0xdf000000)},
                ["sysdec", "sp", "a"]   => {output.push(0xd0000000)},
                ["sysdec", "sp", "b"]   => {output.push(0xd1000000)},
                ["sysdec", "sp", "c"]   => {output.push(0xd2000000)},
                ["sysdec", "sp", "d"]   => {output.push(0xd3000000)},
                ["syscmp", "a"]         => {output.push(0xe1000000)},
                ["syscmp", "b"]         => {output.push(0xe2000000)},
                ["syscmp", "c"]         => {output.push(0xe3000000)},
                ["syscmp", "d"]         => {output.push(0xe4000000)},
                ["syscmp", c]           => {output.push(0xe0000000 | (match c.parse::<u32>() {Ok(v) => v, Err(_e) => {line_err(line_counter, "the first argument must be a 24 bit unsigned constant"); will_panic = true; 0}}))},
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
            use std::io::Write;
            let std_output = std::io::stdout();
            let mut out = std_output.lock();

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
                    0x57 => {{let n = self.sp_reg.overflowing_sub(1); self.c_flag = n.1; self.z_flag = n.1; self.sp_reg = n.0} self.mem[((self.sp_reg-1 << 8) >> 8) as usize] = self.ip_reg; self.ip_reg = ((self.op_bfr) << 8) >> 8}
                    0x58 => {self.ip_reg = self.mem[((self.sp_reg-1 << 8) >> 8) as usize]; {let n = self.sp_reg.overflowing_add(1); self.c_flag = n.1; self.sp_reg = n.0}}
                    0x60 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_add(self.nm_reg); self.a_reg = n.0; self.c_flag = n.1}
                    0x61 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_add(self.nm_reg); self.b_reg = n.0; self.c_flag = n.1}
                    0x62 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_add(self.nm_reg); self.c_reg = n.0; self.c_flag = n.1}
                    0x63 => {let n = (((self.op_bfr) << 8) >> 8).overflowing_add(self.nm_reg); self.d_reg = n.0; self.c_flag = n.1}
                    0x64 => {let n = self.a_reg.overflowing_add(self.nm_reg); self.b_reg = n.0; self.c_flag = n.1}
                    0x65 => {let n = self.a_reg.overflowing_add(self.nm_reg); self.c_reg = n.0; self.c_flag = n.1}
                    0x66 => {let n = self.a_reg.overflowing_add(self.nm_reg); self.d_reg = n.0; self.c_flag = n.1}
                    0x67 => {let n = self.b_reg.overflowing_add(self.nm_reg); self.a_reg = n.0; self.c_flag = n.1}
                    0x68 => {let n = self.b_reg.overflowing_add(self.nm_reg); self.c_reg = n.0; self.c_flag = n.1}
                    0x69 => {let n = self.b_reg.overflowing_add(self.nm_reg); self.d_reg = n.0; self.c_flag = n.1}
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
                    0x80 => {self.a_reg = self.nm_reg & (self.op_bfr << 8) >> 8}
                    0x81 => {self.b_reg = self.nm_reg & (self.op_bfr << 8) >> 8}
                    0x82 => {self.c_reg = self.nm_reg & (self.op_bfr << 8) >> 8}
                    0x83 => {self.d_reg = self.nm_reg & (self.op_bfr << 8) >> 8}
                    0x84 => {self.b_reg = self.nm_reg & self.a_reg}
                    0x85 => {self.c_reg = self.nm_reg & self.a_reg}
                    0x86 => {self.d_reg = self.nm_reg & self.a_reg}
                    0x87 => {self.a_reg = self.nm_reg & self.b_reg}
                    0x88 => {self.c_reg = self.nm_reg & self.b_reg}
                    0x89 => {self.d_reg = self.nm_reg & self.b_reg}
                    0x8a => {self.a_reg = self.nm_reg & self.c_reg}
                    0x8b => {self.b_reg = self.nm_reg & self.c_reg}
                    0x8d => {self.d_reg = self.nm_reg & self.c_reg}
                    0x8c => {self.a_reg = self.nm_reg & self.d_reg}
                    0x8e => {self.b_reg = self.nm_reg & self.d_reg}
                    0x8f => {self.c_reg = self.nm_reg & self.d_reg}
                    0x90 => {self.a_reg = self.nm_reg | (self.op_bfr << 8) >> 8}
                    0x91 => {self.b_reg = self.nm_reg | (self.op_bfr << 8) >> 8}
                    0x92 => {self.c_reg = self.nm_reg | (self.op_bfr << 8) >> 8}
                    0x93 => {self.d_reg = self.nm_reg | (self.op_bfr << 8) >> 8}
                    0x94 => {self.b_reg = self.nm_reg | self.a_reg}
                    0x95 => {self.c_reg = self.nm_reg | self.a_reg}
                    0x96 => {self.d_reg = self.nm_reg | self.a_reg}
                    0x97 => {self.a_reg = self.nm_reg | self.b_reg}
                    0x98 => {self.c_reg = self.nm_reg | self.b_reg}
                    0x99 => {self.d_reg = self.nm_reg | self.b_reg}
                    0x9a => {self.a_reg = self.nm_reg | self.c_reg}
                    0x9b => {self.b_reg = self.nm_reg | self.c_reg}
                    0x9d => {self.d_reg = self.nm_reg | self.c_reg}
                    0x9c => {self.a_reg = self.nm_reg | self.d_reg}
                    0x9e => {self.b_reg = self.nm_reg | self.d_reg}
                    0x9f => {self.c_reg = self.nm_reg | self.d_reg}
                    0xa0 => {self.a_reg = self.nm_reg ^ (self.op_bfr << 8) >> 8}
                    0xa1 => {self.b_reg = self.nm_reg ^ (self.op_bfr << 8) >> 8}
                    0xa2 => {self.c_reg = self.nm_reg ^ (self.op_bfr << 8) >> 8}
                    0xa3 => {self.d_reg = self.nm_reg ^ (self.op_bfr << 8) >> 8}
                    0xa4 => {self.b_reg = self.nm_reg ^ self.a_reg}
                    0xa5 => {self.c_reg = self.nm_reg ^ self.a_reg}
                    0xa6 => {self.d_reg = self.nm_reg ^ self.a_reg}
                    0xa7 => {self.a_reg = self.nm_reg ^ self.b_reg}
                    0xa8 => {self.c_reg = self.nm_reg ^ self.b_reg}
                    0xa9 => {self.d_reg = self.nm_reg ^ self.b_reg}
                    0xaa => {self.a_reg = self.nm_reg ^ self.c_reg}
                    0xab => {self.b_reg = self.nm_reg ^ self.c_reg}
                    0xad => {self.d_reg = self.nm_reg ^ self.c_reg}
                    0xac => {self.a_reg = self.nm_reg ^ self.d_reg}
                    0xae => {self.b_reg = self.nm_reg ^ self.d_reg}
                    0xaf => {self.c_reg = self.nm_reg ^ self.d_reg}
                    0xb0 => {self.a_reg = !(self.op_bfr << 8) >> 8}
                    0xb1 => {self.b_reg = !(self.op_bfr << 8) >> 8}
                    0xb2 => {self.c_reg = !(self.op_bfr << 8) >> 8}
                    0xb3 => {self.d_reg = !(self.op_bfr << 8) >> 8}
                    0xb4 => {self.b_reg = !self.a_reg}
                    0xb5 => {self.c_reg = !self.a_reg}
                    0xb6 => {self.d_reg = !self.a_reg}
                    0xb7 => {self.a_reg = !self.b_reg}
                    0xb8 => {self.c_reg = !self.b_reg}
                    0xb9 => {self.d_reg = !self.b_reg}
                    0xba => {self.a_reg = !self.c_reg}
                    0xbb => {self.b_reg = !self.c_reg}
                    0xbd => {self.d_reg = !self.c_reg}
                    0xbc => {self.a_reg = !self.d_reg}
                    0xbe => {self.b_reg = !self.d_reg}
                    0xbf => {self.c_reg = !self.d_reg}
                    0xc0 => {let n = self.sp_reg.overflowing_add(1); self.c_flag = n.1; self.a_reg = n.0}
                    0xc1 => {let n = self.sp_reg.overflowing_add(1); self.c_flag = n.1; self.b_reg = n.0}
                    0xc2 => {let n = self.sp_reg.overflowing_add(1); self.c_flag = n.1; self.c_reg = n.0}
                    0xc3 => {let n = self.sp_reg.overflowing_add(1); self.c_flag = n.1; self.d_reg = n.0}
                    0xc4 => {let n = self.a_reg.overflowing_add(1); self.c_flag = n.1; self.b_reg = n.0}
                    0xc5 => {let n = self.a_reg.overflowing_add(1); self.c_flag = n.1; self.c_reg = n.0}
                    0xc6 => {let n = self.a_reg.overflowing_add(1); self.c_flag = n.1; self.d_reg = n.0}
                    0xc7 => {let n = self.b_reg.overflowing_add(1); self.c_flag = n.1; self.a_reg = n.0}
                    0xc8 => {let n = self.b_reg.overflowing_add(1); self.c_flag = n.1; self.c_reg = n.0}
                    0xc9 => {let n = self.b_reg.overflowing_add(1); self.c_flag = n.1; self.d_reg = n.0}
                    0xca => {let n = self.c_reg.overflowing_add(1); self.c_flag = n.1; self.a_reg = n.0}
                    0xcb => {let n = self.c_reg.overflowing_add(1); self.c_flag = n.1; self.b_reg = n.0}
                    0xcc => {let n = self.c_reg.overflowing_add(1); self.c_flag = n.1; self.d_reg = n.0}
                    0xcd => {let n = self.d_reg.overflowing_add(1); self.c_flag = n.1; self.a_reg = n.0}
                    0xce => {let n = self.d_reg.overflowing_add(1); self.c_flag = n.1; self.b_reg = n.0}
                    0xcf => {let n = self.d_reg.overflowing_add(1); self.c_flag = n.1; self.c_reg = n.0}
                    0xd0 => {let n = self.sp_reg.overflowing_sub(1); self.c_flag = n.1; self.a_reg = n.0}
                    0xd1 => {let n = self.sp_reg.overflowing_sub(1); self.c_flag = n.1; self.b_reg = n.0}
                    0xd2 => {let n = self.sp_reg.overflowing_sub(1); self.c_flag = n.1; self.c_reg = n.0}
                    0xd3 => {let n = self.sp_reg.overflowing_sub(1); self.c_flag = n.1; self.d_reg = n.0}
                    0xd4 => {let n = self.a_reg.overflowing_sub(1); self.c_flag = n.1; self.b_reg = n.0}
                    0xd5 => {let n = self.a_reg.overflowing_sub(1); self.c_flag = n.1; self.c_reg = n.0}
                    0xd6 => {let n = self.a_reg.overflowing_sub(1); self.c_flag = n.1; self.d_reg = n.0}
                    0xd7 => {let n = self.b_reg.overflowing_sub(1); self.c_flag = n.1; self.a_reg = n.0}
                    0xd8 => {let n = self.b_reg.overflowing_sub(1); self.c_flag = n.1; self.c_reg = n.0}
                    0xd9 => {let n = self.b_reg.overflowing_sub(1); self.c_flag = n.1; self.d_reg = n.0}
                    0xda => {let n = self.c_reg.overflowing_sub(1); self.c_flag = n.1; self.a_reg = n.0}
                    0xdb => {let n = self.c_reg.overflowing_sub(1); self.c_flag = n.1; self.b_reg = n.0}
                    0xdc => {let n = self.c_reg.overflowing_sub(1); self.c_flag = n.1; self.d_reg = n.0}
                    0xdd => {let n = self.d_reg.overflowing_sub(1); self.c_flag = n.1; self.a_reg = n.0}
                    0xde => {let n = self.d_reg.overflowing_sub(1); self.c_flag = n.1; self.b_reg = n.0}
                    0xdf => {let n = self.d_reg.overflowing_sub(1); self.c_flag = n.1; self.c_reg = n.0}
                    0xe0 => {let diff = ((self.op_bfr << 8) >> 8).overflowing_sub(self.nm_reg); if diff.0 > 0 {self.z_flag = false; self.c_flag = true} else if diff.1 {self.z_flag = true; self.c_flag = true} else {self.z_flag = true; self.c_flag = false}}
                    0xe1 => {let diff = self.a_reg.overflowing_sub(self.nm_reg); if diff.0 > 0 {self.z_flag = false; self.c_flag = true} else if diff.1 {self.z_flag = true; self.c_flag = true} else {self.z_flag = true; self.c_flag = false}}
                    0xe2 => {let diff = self.b_reg.overflowing_sub(self.nm_reg); if diff.0 > 0 {self.z_flag = false; self.c_flag = true} else if diff.1 {self.z_flag = true; self.c_flag = true} else {self.z_flag = true; self.c_flag = false}}
                    0xe3 => {let diff = self.c_reg.overflowing_sub(self.nm_reg); if diff.0 > 0 {self.z_flag = false; self.c_flag = true} else if diff.1 {self.z_flag = true; self.c_flag = true} else {self.z_flag = true; self.c_flag = false}}
                    0xe4 => {let diff = self.d_reg.overflowing_sub(self.nm_reg); if diff.0 > 0 {self.z_flag = false; self.c_flag = true} else if diff.1 {self.z_flag = true; self.c_flag = true} else {self.z_flag = true; self.c_flag = false}}
                    _ => {
                        println!("!   ERROR use of unused opcode {} -> {}:{}", self.op_bfr, self.op_bfr >> 24, (self.op_bfr << 8) >> 8);
                        let mut _bffr = String::new();
                        std::io::stdin().read_line(&mut _bffr).unwrap();
                        panic!();
                    }
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

    let code = assembler::assemble_file("example-1.ksm");
    let handler = thread_builder.spawn(move|| {
        let mut computer = emulator::Computer::new(20.0);

        computer.run(code);
    }).unwrap();

    handler.join().unwrap();
}
