use crate::wrapped_instruction::WrappedInstruction;

pub fn format(instructions: &Vec<WrappedInstruction>, accumulation: usize) -> String {
    let mut res = String::from("");
    for instruction in instructions {
        res = format!(
            r#"{res}
{}"#,
            instruction.to_formatted_string(accumulation)
        );
    }
    res.trim_matches('\n').to_string()
}
