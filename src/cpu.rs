use crate::instruction::{ArithmeticTarget, Instruction};

pub struct Cpu;
impl Cpu {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::A => todo!(),
                ArithmeticTarget::B => todo!(),
                ArithmeticTarget::C => todo!(),
                ArithmeticTarget::D => todo!(),
                ArithmeticTarget::E => todo!(),
                ArithmeticTarget::H => todo!(),
                ArithmeticTarget::L => todo!(),
            },
        }
    }
}
