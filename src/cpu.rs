// set <reg> <val> - sets <reg> to the value <val>
// add <val> <val1> - adds the values val and val1 (they can both either be a register or an integer)
// sub <val> <val1> - subtracts the values val and val1 (they can both either be a register or an integer)
// mul <val> <val1> - multiplies the values val and val1 (they can both either be a register or an integer)
// div <val> <val1> - divides the values val and val1 (they can both either be a register or an integer)
// save <val> - saves val to the ram at the adress specified by the ptr register
// load <reg> - loads into reg the value stored at adress ptr
// inc <reg> - increments the register by one
// dec <reg> - decrements the register by one
// start - restarts the program
// print <reg> - prints the register
// printa <reg> - prints the register as an ascii character
// jmp <line> - jumps to line
// jmpif <line> - jumps to line when if flag is true
// smaller <val> <val1> - checks if val is smaller than val1
// greater <val> <val1> - checks if val is greater than val1
// equ <val> <val1> - checks if val is equal to val1
// skipins - skips next instruction when if flag is true
// nop - does nothing

#![allow(unused)]

use std::{fmt::Display, fs::read_to_string, process::exit};

fn throw_err(msg: &str, line: usize) -> ! {
    println!("Error on line {}: {}", line, msg);
    exit(1);
}

fn is_reg(reg: &String) -> bool {
    match reg.as_str() {
        "r1" => {
            true
        },
        "r2" => {
            true
        },
        "r3" => {
            true
        },
        "r4" => {
            true
        },
        "ptr" => {
            true
        },
        "ret" => {
            true
        }
        _ => {
            false
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Set,
    Add,
    Sub,
    Mul,
    Div,
    Save,
    Load,
    Inc,
    Dec,
    Start,
    Print,
    Printa,
    Jmp,
    Jmpif,
    Smaller,
    Greater,
    Equ,
    Skipins,
    Nop
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Op,
    arg1: Option<String>,
    arg2: Option<String>,
}

impl Instruction {
    fn new(op: Op, arg1: Option<String>, arg2: Option<String>) -> Self {
        Instruction {
            op,
            arg1,
            arg2
        }
    }
}

pub struct Cpu {
    instructions: Vec<Instruction>,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    if_flag: bool,
    ret: u16,
    ram: [u16; u16::MAX as usize],
    ptr: u16,
    screen_buf: [[u16; 32]; 32],
    screen: [[u16; 32]; 32],
}

impl Cpu {
    pub fn new(src: &str) -> Self {
        let mut instructions: Vec<Instruction> = Vec::new();

        for (i, line) in read_to_string(src.trim()).unwrap().lines().enumerate() {
            let tokens: Vec<&str> = line.split(" ").collect();
            
            if tokens.len() > 3 || tokens.len() < 1 {
                throw_err("instruction must contain between 1 and 3 tokens", i+1);
            }

            let op: Op = match tokens[0] {
                "set" => {
                    Op::Set
                },
                "add" => {
                    Op::Add
                },
                "sub" => {
                    Op::Sub
                },
                "mul" => {
                    Op::Mul
                },
                "div" => {
                    Op::Div
                },
                "save" => {
                    Op::Save
                },
                "load" => {
                    Op::Load
                },
                "inc" => {
                    Op::Inc
                },
                "dec" => {
                    Op::Dec
                },
                "start" => {
                    Op::Start
                },
                "print" => {
                    Op::Print
                },
                "printa" => {
                    Op::Printa
                },
                "jmp" => {
                    Op::Jmp
                },
                "jmpif" => {
                    Op::Jmpif
                },
                "smaller" => {
                    Op::Smaller
                },
                "greater" => {
                    Op::Greater
                },
                "equ" => {
                    Op::Equ
                },
                "skipins" => {
                    Op::Skipins
                },
                "nop" => {
                    Op::Nop
                },
                "" => {
                    Op::Nop
                },
                _ => {
                    throw_err(format!("unknown token '{}'", tokens[0]).as_str(), i+1);
                }
            };

            let arg1: Option<String> = if tokens.len() > 1 {Some(tokens[1].to_string())} else {None};
            let arg2: Option<String> = if tokens.len() > 2 {Some(tokens[2].to_string())} else {None};

            let instr: Instruction = Instruction::new(op, arg1, arg2);
            instructions.push(instr);
        }

        Cpu {
            instructions,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            if_flag: false,
            ret: 0,
            ram: [0; u16::MAX as usize],
            ptr: 0,
            screen_buf: [[0; 32]; 32],
            screen: [[0; 32]; 32]
        }
    }

    pub fn print_state(&self) {
        println!("[\n   r1: {},\n   r2: {},\n   r3: {},\n   r4: {},\n   if: {},\n   ret: {},\n   ptr: {}\n]", self.r1, self.r2, self.r3, self.r4, self.if_flag, self.ret, self.ptr);
    }

    fn set_reg(&mut self, reg: &String, val: u16) {
        match reg.as_str() {
            "r1" => {
                self.r1 = val;
            },
            "r2" => {
                self.r2 = val;
            },
            "r3" => {
                self.r3 = val;
            },
            "r4" => {
                self.r4 = val;
            },
            "ret" => {
                self.ret = val;
            },
            "ptr" => {
                self.ptr = val;
            }
            _ => {}
        }
    }

    fn get_reg(&mut self, reg: &String) -> u16 {
        match reg.as_str() {
            "r1" => {
                self.r1
            },
            "r2" => {
                self.r2
            },
            "r3" => {
                self.r3
            },
            "r4" => {
                self.r4
            },
            "ret" => {
                self.ret
            },
            "ptr" => {
                self.ptr
            }
            _ => {throw_err("unknown register", 0)}
        }
    }

    pub fn execute(&mut self) {
        let instructions: Vec<Instruction> = self.instructions.clone();

        let mut line: usize = 1;
        let mut i = 0;
        let mut jmp: bool = false;

        while i < instructions.len() {
            let ins: Instruction = instructions[i].clone();
            let op: Op = ins.op;

            match op {
                Op::Set => {
                    if ins.arg1.is_none() || ins.arg2.is_none() {
                        throw_err("the set instruction requires two arguments: 'set <reg> <val>'", line);
                    }

                    let arg1: String = ins.arg1.unwrap();
                    let arg2: String = ins.arg2.unwrap();

                    if !is_reg(&arg1) {
                        throw_err("<reg> must be a register name", line);
                    }

                    let val: u16;

                    if is_reg(&arg2) {
                        val = self.get_reg(&arg2);
                    } else if arg2.parse::<u16>().is_ok() {
                        val = arg2.parse::<u16>().unwrap();
                    } else {
                        throw_err("<val> must either be a register name or a 16 bit integer", line);
                    }

                    self.set_reg(&arg1, val);
                },
                Op::Add | Op::Sub | Op::Mul | Op::Div => {
                    if ins.arg1.is_none() || ins.arg2.is_none() {
                        throw_err("the arithmetic operator instructions requires two arguments: 'operator <val> <val1>'", line);
                    }

                    let arg1: String = ins.arg1.unwrap();
                    let arg2: String = ins.arg2.unwrap();

                    let val: u16;
                    let val1: u16;

                    if is_reg(&arg1) {
                        val = self.get_reg(&arg1);
                    } else if arg1.parse::<u16>().is_ok() {
                        val = arg1.parse::<u16>().unwrap();
                    } else {
                        throw_err("<val> must either be a register name or a 16 bit integer", line)
                    }


                    if is_reg(&arg2) {
                        val1 = self.get_reg(&arg2);
                    } else if arg2.parse::<u16>().is_ok() {
                        val1 = arg2.parse::<u16>().unwrap();
                    } else {
                        throw_err("<val1> must either be a register name or a 16 bit integer", line);
                    }

                    self.ret = match op {
                        Op::Add => {
                            val.checked_add(val1).unwrap_or(u16::MAX)
                        },
                        Op::Sub => {
                            val.checked_sub(val1).unwrap_or(u16::MAX)
                        },
                        Op::Mul => {
                            val.checked_mul(val1).unwrap_or(u16::MAX)
                        },
                        Op::Div => {
                            val.checked_div(val1).unwrap_or(u16::MAX)
                        },
                        _ => {throw_err("unknown operator", line)}
                    };
                },
                Op::Save => {
                    if ins.arg1.is_none() || ins.arg2.is_some() {
                        throw_err("save instruction requires 1 argument: 'save <val>'", line)
                    }

                    let arg1: String = ins.arg1.unwrap();
                    let val: u16;

                    if is_reg(&arg1) {
                        val = self.get_reg(&arg1);
                    } else if arg1.parse::<u16>().is_ok() {
                        val = arg1.parse::<u16>().unwrap();
                    } else {
                        throw_err("<val> must be either a register or a 16 bit integer", line);
                    }

                    self.ram[self.ptr as usize] = val;
                },
                Op::Load => {
                    if ins.arg1.is_none() || ins.arg2.is_some() {
                        throw_err("load instruction requires 1 argument: 'load <reg>'", line)
                    }

                    let arg1: String = ins.arg1.unwrap();
                    let val: u16;

                    if is_reg(&arg1) {
                        self.set_reg(&arg1, self.ram[self.ptr as usize]);
                    } else {
                        throw_err("<reg> must be a register name", line);
                    }
                },
                Op::Inc => {
                    if ins.arg1.is_none() || ins.arg2.is_some() {
                        throw_err("inc instruction requires 1 argument: 'inc <reg>'", line)
                    }

                    let arg1: String = ins.arg1.unwrap();
                    let val: u16;

                    if is_reg(&arg1) {
                        let val: u16 = self.get_reg(&arg1).checked_add(1).unwrap_or(u16::MAX);
                        self.set_reg(&arg1, val);
                    } else {
                        throw_err("<reg> must be a register name", line);
                    }
                },
                Op::Dec => {
                    if ins.arg1.is_none() || ins.arg2.is_some() {
                        throw_err("dec instruction requires 1 argument: 'dec <reg>'", line)
                    }

                    let arg1: String = ins.arg1.unwrap();
                    let val: u16;

                    if is_reg(&arg1) {
                        let val: u16 = self.get_reg(&arg1).checked_sub(1).unwrap_or(u16::MAX);
                        self.set_reg(&arg1, val);
                    } else {
                        throw_err("<reg> must be a register name", line);
                    }
                },
                Op::Start => {
                    if ins.arg1.is_some() || ins.arg2.is_some() {
                        throw_err("the start instruction doesn't take any argument: 'start'", line);
                    }

                    i = 0;
                    jmp = true;
                },
                Op::Print => {
                    if ins.arg1.is_none() || ins.arg2.is_some() {
                        throw_err("print instruction requires 1 argument: 'print <reg>'", line)
                    }

                    let arg1: String = ins.arg1.unwrap();

                    if is_reg(&arg1) {
                        println!("{}", self.get_reg(&arg1));
                    } else {
                        throw_err("<reg> must be a register name", line);
                    }
                },
                Op::Printa => {
                    if ins.arg1.is_none() || ins.arg2.is_some() {
                        throw_err("printa instruction requires 1 argument: 'printa <reg>'", line)
                    }

                    let arg1: String = ins.arg1.unwrap();

                    if is_reg(&arg1) {
                        println!("{}", self.get_reg(&arg1) as u8 as char);
                    } else {
                        throw_err("<reg> must be a register name", line);
                    }
                },
                Op::Jmp => {
                    if ins.arg1.is_none() || ins.arg2.is_some() {
                        throw_err("the jmp instruction takes 1 argument: 'jmp <line>'", line);
                    }

                    let arg1: String = ins.arg1.unwrap();
                    let val: u16;

                    if arg1.parse::<u16>().is_ok() {
                        val = arg1.parse::<u16>().unwrap();
                    } else {
                        throw_err("<line> can only be an integer", line)
                    }

                    i = (val - 1) as usize;
                    jmp = true;
                },
                Op::Jmpif => {
                    if ins.arg1.is_none() || ins.arg2.is_some() {
                        throw_err("the jmpif instruction takes 1 argument: 'jmpif <line>'", line);
                    }

                    let arg1: String = ins.arg1.unwrap();
                    let val: u16;

                    if arg1.parse::<u16>().is_ok() {
                        val = arg1.parse::<u16>().unwrap();
                    } else {
                        throw_err("<line> can only be an integer", line)
                    }

                    if self.if_flag == true {
                        i = (val - 1) as usize;
                        jmp = true;
                        self.if_flag = false;
                    }
                },
                Op::Smaller | Op::Greater | Op::Equ => {
                    if ins.arg1.is_none() || ins.arg2.is_none() {
                        throw_err("the comparison operator instructions requires two arguments: 'operator <val> <val1>'", line);
                    }

                    let arg1: String = ins.arg1.unwrap();
                    let arg2: String = ins.arg2.unwrap();

                    let val: u16;
                    let val1: u16;

                    if is_reg(&arg1) {
                        val = self.get_reg(&arg1);
                    } else if arg1.parse::<u16>().is_ok() {
                        val = arg1.parse::<u16>().unwrap();
                    } else {
                        throw_err("<val> must either be a register name or a 16 bit integer", line)
                    }


                    if is_reg(&arg2) {
                        val1 = self.get_reg(&arg2);
                    } else if arg2.parse::<u16>().is_ok() {
                        val1 = arg2.parse::<u16>().unwrap();
                    } else {
                        throw_err("<val1> must either be a register name or a 16 bit integer", line);
                    }

                    self.if_flag = match op {
                        Op::Smaller => {
                            if val < val1 {
                                true
                            } else {
                                false
                            }
                        },
                        Op::Greater => {
                            if val > val1 {
                                true
                            } else {
                                false
                            }
                        },
                        Op::Equ => {
                            if val == val1 {
                                true
                            } else {
                                false
                            }
                        },
                        _ => {throw_err("unknown comparator", line)}
                    }
                },
                Op::Skipins => {
                    if ins.arg1.is_some() || ins.arg2.is_some() {
                        throw_err("the skipins instruction doesn't take any argument: 'skipins'", line);
                    }

                    if self.if_flag == true {
                        i += 1;
                        self.if_flag = false;
                    }
                },
                Op::Nop => {
                    if ins.arg1.is_some() || ins.arg2.is_some() {
                        throw_err("the nop instruction doesn't take any argument: 'nop'", line);
                    }
                }
            }
        
            i += if jmp == true {jmp = false; 0} else {1};
            line = i + 1;
        }
    }
}