use crate::formatter::format;
use crate::instruction::Instruction;
pub enum InstructionWrapper {
    Comment(String),
    Instruction {
        instruction: Instruction,
        comment: Option<String>,
    },
}

impl InstructionWrapper {
    pub fn to_formatted_string(&self, depth: usize, tab: &str) -> String {
        let space = tab.repeat(depth);
        match self {
            InstructionWrapper::Comment(cmt) => format!("{space}# {}", cmt),
            InstructionWrapper::Instruction {
                instruction,
                comment,
            } => {
                let formatted_string = match instruction {
                    ////////////////////////////////////////////////
                    ////////////////Control Structures//////////////
                    ////////////////////////////////////////////////
                    Instruction::If { instr1, instr2 } => {
                        let label = instruction.get_label();
                        let space = tab.repeat(depth);
                        let formatted_instr1 = format(instr1, depth + 1, tab);
                        let formatted_instr2 = format(instr2, depth + 1, tab);
                        format!(
                            r#"{space}{label}
{space}{{
{formatted_instr1}
{space}}}
{space}{{
{formatted_instr2}
{space}}}"#
                        )
                    }
                    Instruction::IfCons { .. } => todo!(),
                    Instruction::IfLeft { .. } => todo!(),
                    Instruction::IfNone { .. } => todo!(),
                    //ITER inster,
                    //LAMBDA ty1 ty2 instr,
                    Instruction::Loop { .. } => todo!(),
                    Instruction::LoopLeft { .. } => todo!(),
                    //instr1 ; instr2,
                    //{},
                    ////////////////////////////////////////////////
                    //////////Operations on data structures/////////
                    ////////////////////////////////////////////////
                    ////////////////////////////////////////////////
                    /////////////Blockchain operations//////////////
                    ////////////////////////////////////////////////
                    //CREATE_CONTRACT { parameter ty1; storage ty2; code instr1 },
                    ////////////////////////////////////////////////
                    ////////////Operations on tickets///////////////
                    ////////////////////////////////////////////////
                    ////////////////////////////////////////////////
                    ////////////Cryptographic operations////////////
                    ////////////////////////////////////////////////
                    ////////////////////////////////////////////////
                    //////////////Boolean operations////////////////
                    ////////////////////////////////////////////////
                    ////////////////////////////////////////////////
                    ////////////Arithmetic operations///////////////
                    ////////////////////////////////////////////////
                    ////////////////////////////////////////////////
                    /////////////Stack manipulation/////////////////
                    ////////////////////////////////////////////////
                    Instruction::Push { ty, val } => {
                        format!(
                            "{space}{} {} {}",
                            instruction.get_label(),
                            ty.to_string(),
                            val.to_string()
                        )
                    }
                    _ => format!("{space}{}", instruction.get_label()),
                };
                match comment {
                    Some(s) => format!("{formatted_string}; # {s}"),
                    None => format!("{formatted_string};"),
                }
            }
        }
    }
}
