use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt;
use std::io::{self, BufRead};

use itertools::Itertools;

pub enum Operand {
    Immediate(i64),
    W,
    X,
    Y,
    Z,
}

impl Operand {
    fn parse(operand: &str) -> Self {
        match operand {
            "w" => Operand::W,
            "x" => Operand::X,
            "y" => Operand::Y,
            "z" => Operand::Z,
            _ => Operand::Immediate(operand.parse().unwrap()),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::W => write!(f, "w"),
            Operand::X => write!(f, "x"),
            Operand::Y => write!(f, "y"),
            Operand::Z => write!(f, "z"),
            Operand::Immediate(v) => write!(f, "{}", v),
        }
    }
}

enum Instruction {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

impl Instruction {
    fn parse(instruction: &str) -> Self {
        match &instruction[0..3] {
            "inp" => Instruction::Inp(Operand::parse(&instruction[4..])),
            _ => {
                let (op1, op2) = instruction[4..].split_whitespace().map(Operand::parse).collect_tuple().unwrap();
                match &instruction[0..3] {
                    "add" => Instruction::Add(op1, op2),
                    "mul" => Instruction::Mul(op1, op2),
                    "div" => Instruction::Div(op1, op2),
                    "mod" => Instruction::Mod(op1, op2),
                    "eql" => Instruction::Eql(op1, op2),
                    _ => panic!("Unknown mnemonic!"),
                }
            },
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Inp(op) => write!(f, "inp {}", op),
            Instruction::Add(op1, op2) => write!(f, "add {} {}", op1, op2),
            Instruction::Mul(op1, op2) => write!(f, "mul {} {}", op1, op2),
            Instruction::Div(op1, op2) => write!(f, "div {} {}", op1, op2),
            Instruction::Mod(op1, op2) => write!(f, "mod {} {}", op1, op2),
            Instruction::Eql(op1, op2) => write!(f, "eql {} {}", op1, op2),
        }
    }
}

pub struct Program {
    instructions: Vec<Instruction>,
    sym2loc: HashMap<String, usize>,
    loc2sym: HashMap<usize, String>,
}

impl Program {
    fn new(instructions: Vec<Instruction>, symbols: HashMap<String, usize>) -> Self {
        let loc2sym = symbols.iter().map(|(k,v)| (*v,k.clone())).collect();
        Program {
            instructions,
            sym2loc: symbols,
            loc2sym,
        }
    }

    pub fn loc(&self, sym: &str) -> Option<usize> {
        if let Some(loc) = self.sym2loc.get(sym) {
            Some(*loc)
        } else {
            None
        }
    }

    pub fn sym(&self, loc: usize) -> Option<&str> {
        if let Some(sym) = self.loc2sym.get(&loc) {
            Some(&sym[..])
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum AluError {
    TtlExpired,
    Breakpoint(Box<dyn Error>),
}

impl fmt::Display for AluError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TtlExpired => write!(f, "TTL expired"),
            Self::Breakpoint(_) => write!(f, "breakpoint triggered"),
        }
    }
}

impl Error for AluError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::TtlExpired => None,
            Self::Breakpoint(e) => Some(&**e),
        }
    }
}

/// The result of an ALU execution.
type AluResult = Result<i64, AluError>;

/// A breakpoint callback called upon reaching a location of an ALU program. Can return Err to
/// halt execution of the ALU program.
type BpResult = Result<(), Box<dyn Error>>;
type BpCallback<I> = dyn FnMut(&Alu<I>) -> BpResult;

pub struct Alu<'p, I: Iterator<Item=u8>> {
    /// IP, the Instruction Pointer <3
    pub ip: usize,
    pub w: i64,
    pub x: i64,
    pub y: i64,
    pub z: i64,
    program: Option<&'p Program>,
    stdin: Option<I>,
    debug: bool,
    ttl: Option<usize>,
    watchpoints: BTreeMap<usize, Vec<Operand>>,
    breakpoints: BTreeMap<usize, Vec<Box<RefCell<BpCallback<I>>>>>,
}

impl<'p, I: Iterator<Item=u8>> Alu<'p, I> {
    pub fn new() -> Self {
        Alu {
            ip: 0,
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            program: None,
            stdin: None,
            debug: false,
            ttl: None,
            watchpoints: BTreeMap::new(),
            breakpoints: BTreeMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.w = 0;
        self.x = 0;
        self.y = 0;
        self.z = 0;
        self.stdin = None;
    }

    pub fn load(&mut self, program: &'p Program) {
        self.program = Some(program);
    }

    pub fn get_program(&self) -> Option<&Program> {
        self.program
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn set_ttl(&mut self, ttl: Option<usize>) {
        self.ttl = ttl;
    }

    pub fn add_watchpoint(&mut self, loc: usize, op: Operand) {
        self.watchpoints.entry(loc).or_insert_with(|| Vec::new()).push(op);
    }

    pub fn add_breakpoint(&mut self, loc: usize, callback: Box<RefCell<BpCallback<I>>>) {
        self.breakpoints.entry(loc).or_insert_with(|| Vec::new()).push(callback);
    }

    fn variable(&mut self, op: &Operand) -> &mut i64 {
        match op {
            Operand::Immediate(_) => panic!("Expected variable, got immediate!"),
            Operand::W => &mut self.w,
            Operand::X => &mut self.x,
            Operand::Y => &mut self.y,
            Operand::Z => &mut self.z,
        }
    }

    fn value(&self, op: &Operand) -> i64 {
        match op {
            Operand::Immediate(v) => *v,
            Operand::W => self.w,
            Operand::X => self.x,
            Operand::Y => self.y,
            Operand::Z => self.z,
        }
    }

    fn input<'o>(&mut self, op: &'o Operand) -> &'o Operand {
        if let Some(ref mut stdin) = self.stdin {
            *self.variable(op) = stdin.next().expect("Ran out of input!") as i64;
        } else {
            panic!("No input!");
        }
        op
    }

    fn add<'o>(&mut self, op1: &'o Operand, op2: &'o Operand) -> &'o Operand {
        let src = self.value(op2);
        let dst = self.variable(op1);
        *dst += src;
        op1
    }

    fn multiply<'o>(&mut self, op1: &'o Operand, op2: &'o Operand) -> &'o Operand {
        let src = self.value(op2);
        let dst = self.variable(op1);
        *dst *= src;
        op1
    }

    fn divide<'o>(&mut self, op1: &'o Operand, op2: &'o Operand) -> &'o Operand {
        let src = self.value(op2);
        let dst = self.variable(op1);
        *dst /= src;
        op1
    }

    fn modulo<'o>(&mut self, op1: &'o Operand, op2: &'o Operand) -> &'o Operand {
        let src = self.value(op2);
        let dst = self.variable(op1);
        *dst %= src;
        op1
    }

    fn equal<'o>(&mut self, op1: &'o Operand, op2: &'o Operand) -> &'o Operand {
        let src = self.value(op2);
        let dst = self.variable(op1);
        *dst = if *dst == src { 1 } else { 0 };
        op1
    }

    fn trigger_watchpoints(&self) {
        if let Some(watches) = self.watchpoints.get(&self.ip) {
            let sym = self.program.unwrap().sym(self.ip).map_or_else(|| format!("{}", self.ip), str::to_owned);
            for w in watches {
                eprintln!(" <{}># {} = {}", sym, w, self.value(w));
            }
        }
    }

    /// Calls all breakpoint callbacks defined for the current IP. If any of them return an error,
    /// stop and return the error to indicate exexution must stop.
    fn trigger_breakpoints(&self) -> BpResult {
        if let Some(breaks) = self.breakpoints.get(&self.ip) {
            for b in breaks {
                (*b).borrow_mut()(self)?;
            }
        }
        Ok(())
    }

    fn step(&mut self) -> AluResult {
        self.trigger_watchpoints();
        if let Err(e) = self.trigger_breakpoints() {
            return Err(AluError::Breakpoint(e));
        }

        let insn = &self.program.unwrap().instructions[self.ip];
        let dst_op = match insn {
            Instruction::Inp(op) => self.input(op),
            Instruction::Add(op1, op2) => self.add(op1, op2),
            Instruction::Mul(op1, op2) => self.multiply(op1, op2),
            Instruction::Div(op1, op2) => self.divide(op1, op2),
            Instruction::Mod(op1, op2) => self.modulo(op1, op2),
            Instruction::Eql(op1, op2) => self.equal(op1, op2),
        };

        if self.debug {
            eprintln!(" {}\n   -> {} = {}", insn, dst_op, self.value(dst_op));
        }

        Ok(self.z)
    }

    pub fn execute<C>(&mut self, stdin: C) -> AluResult
    where
        C: IntoIterator<Item = u8, IntoIter = I>,
    {
        let program = self.program.expect("No program loaded!");
        // Consume the input.
        self.stdin = Some(stdin.into_iter());

        let mut res = self.z;
        while self.ip != program.instructions.len() {
            if self.ttl.unwrap_or(1) == 0 {
                return Err(AluError::TtlExpired);
            }

            // Execute one instruction.
            res = self.step()?;

            // Advance to the next instruction.
            self.ip += 1;
            if let Some(ref mut ttl) = self.ttl {
                *ttl -= 1;
            }
        }

        // Trigger any watch/breakpoints set at the end of the program.
        self.trigger_watchpoints();
        if let Err(e) = self.trigger_breakpoints() {
            return Err(AluError::Breakpoint(e));
        }

        Ok(res)
    }
}

fn digitize(mut num: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    while num > 0 {
        digits.push((num % 10) as u8);
        num /= 10;
    }
    digits.reverse();
    digits
}

#[derive(Debug)]
struct InputCheckBreak {
    loc: usize,
    required: i64,
}

impl fmt::Display for InputCheckBreak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "required input value {} at loc {}", self.required, self.loc)
    }
}

impl Error for InputCheckBreak {}

pub fn brute_force_monad() -> Vec<u64> {
    let program = parse_stdin();
    let mut alu: Alu<std::vec::IntoIter<u8>> = Alu::new();
    alu.load(&program);

    // alu.set_debug(true);
    // alu.set_ttl(20);
    // for i in 0..14 {
    //     alu.add_watchpoint(program.loc(&format!("round{}", i)).unwrap(), Operand::Z);
    // }
    // alu.add_watchpoint(program.loc("end").unwrap(), Operand::Z);

    // There are 2 kinds of "rounds" in the ALU program:
    // * those that can only multiply Z by 26 (`div z 1`, and W can't match X in `eql x w` due to
    //   positive offset of X)
    // * those that can either keep Z about the same, or divide it by 26 (`div z 26`, and W can
    //   match X in `eql x w` due to negative offset of X)
    // Since there are 7 of each, to reach Z = 0 at the end of the program, we need all rounds that
    // can divide Z by 26 to actually do so. This means that for these rounds, W (the input digit)
    // must match X. This makes it possible to compute all possible model numbers by brute-forcing
    // all digits in "mul z 26" rounds, and computing the matching digits in all "div z 26"
    // rounds.
    // To do this we'll run the program with breakpoints set to inspect its state and find out the
    // correct input for the "div z 26" rounds.
    // This is probably not very efficient since we're running the whole program from the start for
    // every input. ¯\_(ツ)_/¯

    let inputcheck_cb = |alu: &Alu<_>| -> BpResult {
        // The actual model number can't contain any 0s, so we use them for undetermined digits.
        if alu.w == 0 {
            Err(Box::new(InputCheckBreak {
                loc: alu.ip,
                required: alu.x,
            }))
        } else {
            Ok(())
        }
    };
    alu.add_breakpoint(program.loc("round3_x").unwrap(), Box::new(RefCell::new(inputcheck_cb)));
    alu.add_breakpoint(program.loc("round5_x").unwrap(), Box::new(RefCell::new(inputcheck_cb)));
    alu.add_breakpoint(program.loc("round8_x").unwrap(), Box::new(RefCell::new(inputcheck_cb)));
    alu.add_breakpoint(program.loc("round10_x").unwrap(), Box::new(RefCell::new(inputcheck_cb)));
    alu.add_breakpoint(program.loc("round11_x").unwrap(), Box::new(RefCell::new(inputcheck_cb)));
    alu.add_breakpoint(program.loc("round12_x").unwrap(), Box::new(RefCell::new(inputcheck_cb)));
    alu.add_breakpoint(program.loc("round13_x").unwrap(), Box::new(RefCell::new(inputcheck_cb)));

    // We brute-force the consecutive digits corresponding to rounds that do *not* do a `div z 26`
    // operation.
    // These groups of digits are described below by their position in the model number (i.e. the
    // round index of the first digit of the group), and number of digits in the group. The last
    // groups have 0 digits: indeed, rounds 10, 11, 12 & 13 all do a `div z 26` operation, so
    // there's nothing to brute-force for these "groups", we just need to check what the required
    // value for W is (and whether it is within the digit input range).
    let digit_groups = [
        (0, 3),
        (4, 1),
        (6, 2),
        (9, 1),
        (11, 0),
        (12, 0),
        (13, 0),
    ];
    let mut models = vec![0u64];
    for (index, num_digits) in &digit_groups {
        if *num_digits > 0 {
            models = models.iter().map(|m| find_model_digits(&mut alu, *index, *num_digits, *m)).flatten().collect();
            println!("Found {} {}-digit prefixes.", models.len(), *index + *num_digits);
        } else {
            models = models.iter().filter_map(|m| check_model_prefix(&mut alu, *index, *m)).collect();
            println!("Retained {} {}-digit prefixes.", models.len(), *index + *num_digits);
        }
    }

    models.retain(|m| check_model(&mut alu, *m));
    println!("Found {} valid model numbers.", models.len());

    models
}

fn find_model_digits(alu: &mut Alu<std::vec::IntoIter<u8>>, index: usize, num_digits: usize, prefix: u64) -> Vec<u64> {
    let factor = 10u64.pow((14-index-num_digits) as u32);
    let mut prefixes = Vec::new();
    // Iterate over all possible values of the digit group (0 isn't a possible digit value).
    for digits in (0..num_digits).map(|_| 1..=9).multi_cartesian_product() {
        // Generate and check the corresponding partial model number.
        let model = digits.iter().enumerate().map(|(i,d)| d*10u64.pow(i as u32)).sum::<u64>() * factor + prefix;

        if let Some(model) = check_model_prefix(alu, index + num_digits, model) {
            prefixes.push(model);
        }
    }

    prefixes
}

fn check_model_prefix(alu: &mut Alu<std::vec::IntoIter<u8>>, index: usize, prefix: u64) -> Option<u64> {
    // eprintln!("Checking partial model number {:14}...", prefix);
    let input = digitize(prefix);
    alu.reset();

    if let Err(err) = alu.execute(input) {
        if let AluError::Breakpoint(err) = err {
            // Tripped a breakpoint, check the required value for the digit after the digit group.
            let err = err.downcast_ref::<InputCheckBreak>().unwrap();
            // Ensure we tripped the right breakpoint.
            assert_eq!(err.loc, alu.get_program().unwrap().loc(&format!("round{}_x", index)).unwrap());
            if 1 <= err.required && err.required <= 9 {
                // Found a satisfiable prefix.
                let factor = 10u64.pow((14-index) as u32);
                return Some(prefix + err.required as u64 * factor / 10);
            }
        } else {
            panic!("Unexpected ALU error: {}", err);
        }
    }

    None
}

fn check_model(alu: &mut Alu<std::vec::IntoIter<u8>>, model: u64) -> bool {
    let input = digitize(model);
    alu.reset();
    if let Ok(res) = alu.execute(input) {
        res == 0
    } else {
        // We should only be getting full model numbers, so no breakpoint should trigger.
        panic!();
    }
}

pub fn parse_stdin() -> Program {
    parse_lines(io::stdin().lock().lines().flatten())
}

pub fn parse_lines<I>(lines: I) -> Program 
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    let mut symbols = HashMap::new();
    let mut loc: usize = 0;
    let instructions = lines.into_iter().filter_map(|l| {
        // Non-standard ALU ASM syntax extension: comments!
        let l = l.borrow().split('#').nth(0).unwrap().trim();
        if l.is_empty() {
            None
        } else if l.ends_with(":") {
            // Non-standard ALU ASM syntax extension: labels.
            let label = &l[..l.len()-1];
            if symbols.insert(label.to_owned(), loc).is_some() {
                eprintln!("Multiply-defined label: {}", label);
            }
            None
        } else {
            loc += 1;
            Some(Instruction::parse(&l))
        }
    }).collect();

    Program::new(instructions, symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exec(program: &str, input: &[u8], output: [Option<i64>; 4]) {
        let program = parse_lines(program.lines());
        let mut alu = Alu::new();
        alu.execute(&program, input.iter().copied());
        if let Some(w) = output[0] {
            assert_eq!(alu.w, w);
        }
        if let Some(x) = output[1] {
            assert_eq!(alu.x, x);
        }
        if let Some(y) = output[2] {
            assert_eq!(alu.y, y);
        }
        if let Some(z) = output[3] {
            assert_eq!(alu.z, z);
        }
    }

    #[test]
    fn negate() {
        exec(r"
                inp x
                mul x -1
            ",
            &[42],
            [None, Some(-42), None, None]
        );
    }

    #[test]
    fn compare() {
        let program = r"
            inp z
            inp x
            mul z 3
            eql z x
        ";
        exec(program,
            &[1, 6],
            [None, None, None, Some(0)]
        );
        exec(program,
            &[2, 6],
            [None, None, None, Some(1)]
        );
        exec(program,
            &[3, 6],
            [None, None, None, Some(0)]
        );
    }

    #[test]
    fn binary() {
        let program = r"
            inp w
            add z w
            mod z 2
            div w 2
            add y w
            mod y 2
            div w 2
            add x w
            mod x 2
            div w 2
            mod w 2
        ";
        exec(program,
            &[0],
            [Some(0), Some(0), Some(0), Some(0)]
        );
        exec(program,
            &[1],
            [Some(0), Some(0), Some(0), Some(1)]
        );
        exec(program,
            &[2],
            [Some(0), Some(0), Some(1), Some(0)]
        );
        exec(program,
            &[4],
            [Some(0), Some(1), Some(0), Some(0)]
        );
        exec(program,
            &[8],
            [Some(1), Some(0), Some(0), Some(0)]
        );
        exec(program,
            &[0xf],
            [Some(1), Some(1), Some(1), Some(1)]
        );
    }
}
