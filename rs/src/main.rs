mod binary;
mod vm;
use std::env;
use std::fs;

use binary::chunk::Constant;
use vm::instruction::Instruction;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let data = fs::read(args[1].clone()).expect("cannot read file");
        let proto = binary::undump(data);
        list(&proto);
    }
}

fn list(proto: &binary::chunk::Prototype) {
    print_header(&proto);
    print_code(&proto);
    print_detail(&proto);
    for p in proto.protos.iter() {
        list(p);
    }
}

fn print_header(proto: &binary::chunk::Prototype) {
    let func_type = if proto.line_defined <= 0 {
        "main"
    } else {
        "function"
    };

    let vararg_flag = if proto.is_vararg > 0 { "+" } else { "" };

    println!(
        "{func_type} <{}:{}, {}> ({} instructions)",
        proto.source,
        proto.line_defined,
        proto.last_line_defined,
        proto.code.len()
    );

    print!(
        "{}{} params, {} slots, {} upvalues, ",
        proto.num_params,
        vararg_flag,
        proto.max_stack_size,
        proto.upvalues.len()
    );
    println!(
        "{} locals, {} constants, {} functions",
        proto.loc_vars.len(),
        proto.constants.len(),
        proto.protos.len()
    );
}

fn print_code(proto: &binary::chunk::Prototype) {
    for (i, c) in proto.code.iter().enumerate() {
        let line = if proto.line_info.len() > 0 {
            format!("{}", proto.line_info[i])
        } else {
            "-".to_string()
        };

        let instruction = c.clone();
        print!("\t{}\t[{}]\t{} \t", i + 1, line, instruction.op_name());
        print_operands(instruction);
        println!();
    }
}

fn print_operands(instruction: u32) {
    match instruction.op_mode() {
        vm::opcode::OP_MODE_ABC => {
            let (a, b, c) = instruction.abc();
            print!("{}", a);
            if instruction.b_arg_mode() != vm::opcode::OP_ARG_N {
                if b > 0xFF {
                    print!(" {}", -1 - (b & 0xFF));
                } else {
                    print!(" {}", b);
                }
            }

            if instruction.c_arg_mode() != vm::opcode::OP_ARG_N {
                if c > 0xFF {
                    print!(" {}", -1 - (c & 0xFF));
                } else {
                    print!(" {}", c);
                }
            }
        }
        vm::opcode::OP_MODE_ABX => {
            let (a, bx) = instruction.abx();
            print!("{}", a);
            if instruction.b_arg_mode() == vm::opcode::OP_ARG_K {
                print!(" {}", -1 - bx);
            } else if instruction.b_arg_mode() == vm::opcode::OP_ARG_U {
                print!(" {}", bx)
            }
        }
        vm::opcode::OP_MODE_ASBX => {
            let (a, sbx) = instruction.asbx();
            print!("{} {}", a, sbx);
        }
        vm::opcode::OP_MODE_AX => {
            let ax = instruction.ax();
            print!("{}", -1 - ax)
        }
        _ => ()
    }
}

fn print_detail(proto: &binary::chunk::Prototype) {
    println!("constants ({}):", proto.constants.len());
    for (i, c) in proto.constants.iter().enumerate() {
        println!("\t{}\t{}", i + 1, constant_to_string(c));
    }

    println!("locals ({}):", proto.loc_vars.len());
    for (i, l) in proto.loc_vars.iter().enumerate() {
        println!(
            "\t{}\t{}\t{}\t{}",
            i,
            l.var_name,
            l.start_pc + 1,
            l.end_pc + 1
        );
    }

    println!("upvalues ({}):", proto.upvalues.len());
    for (i, u) in proto.upvalues.iter().enumerate() {
        println!(
            "\t{}\t{}\t{}\t{}",
            i,
            upvalue_name(proto, i),
            u.instack,
            u.idx
        );
    }
}

fn constant_to_string(constant: &Constant) -> String {
    match constant {
        Constant::Nil => "nil".to_string(),
        Constant::Boolean(b) => b.to_string(),
        Constant::Number(n) => n.to_string(),
        Constant::Integer(i) => i.to_string(),
        Constant::Str(s) => format!("\"{}\"", s),
    }
}

fn upvalue_name(proto: &binary::chunk::Prototype, index: usize) -> String {
    if proto.upvalue_names.len() >= index {
        proto.upvalue_names[index].clone()
    } else {
        "-".to_string()
    }
}
