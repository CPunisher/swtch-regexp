use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Inst {
    Char(char),
    Split(Vec<usize>),
    Jump(usize),
    Match,
    Noop,
}

pub enum InstBlock {
    Inst(Inst),
    InstNodeIndex(usize),
}

pub struct InstNode(pub Vec<InstBlock>);

impl InstNode {
    pub fn single_inst(inst: Inst) -> Self {
        InstNode(vec![InstBlock::Inst(inst)])
    }
}

impl Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inst::Char(c) => write!(f, "char {}", c),
            Inst::Split(ids) => write!(
                f,
                "split {}",
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Inst::Jump(id) => write!(f, "jmp {}", id),
            Inst::Match => write!(f, "match"),
            Inst::Noop => write!(f, "noop"),
        }
    }
}
