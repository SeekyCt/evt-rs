use std::fmt;

use crate::{Script, opcode::Opcode};

#[derive(Default)]
pub struct PrintSettings {
    pub macros: bool,
}

pub fn print_evt<W>(writer: &mut W, evt: Script, settings: PrintSettings) -> fmt::Result
where
    W: fmt::Write + ?Sized,
{
    let mut indent = 0;
    for instr in evt {
        let opcode: Opcode = instr.opcode;

        // Unindent for this line
        indent = opcode.apply_unindent(indent);

        for _ in 0..indent {
            write!(writer, "    ")?;
        }

        let name = if settings.macros {
            opcode.c_macro()
        } else {
            opcode.name()
        };
        write!(writer, "{}", name)?;

        for arg in instr.args {
            write!(writer, " {:?}", &arg)?;
        }

        write!(writer, "\n")?;

        // Indent for next line
        indent = opcode.apply_indent(indent);
    }
    Ok(())
}
