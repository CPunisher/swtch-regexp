use super::inst::Inst;

pub struct Interpreter {
    prog: Vec<Inst>,
}

impl Interpreter {
    pub fn new(prog: Vec<Inst>) -> Self {
        Self { prog }
    }

    pub fn thompson_vm(&self, input: &str) -> bool {
        let mut clist = vec![0 as usize];

        for sp in input.chars().chain(std::iter::once('\0')) {
            let mut nlist = vec![];
            while let Some(pc) = clist.pop() {
                match &self.prog[pc] {
                    Inst::Char(c) => {
                        if sp != *c {
                            continue;
                        }
                        nlist.push(pc + 1);
                    }
                    Inst::Jump(pc1) => {
                        clist.push(*pc1);
                    }
                    Inst::Split(pc_list) => {
                        for &pc in pc_list {
                            clist.push(pc);
                        }
                    }
                    Inst::Match => return true,
                    Inst::Noop => {
                        clist.push(pc + 1);
                    }
                }
            }
            clist = nlist;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::{compile, compiler::print_prog};

    use super::*;
    #[test]
    fn test_backtracking_vm() {
        print_prog(&compile("((a|b)*)"));
        let interpreter = Interpreter::new(compile("(a+)"));
        assert!(interpreter.thompson_vm("a"));

        let interpreter = Interpreter::new(compile("((a|b)*)"));
        assert!(interpreter.thompson_vm("a"));
        assert!(interpreter.thompson_vm("b"));
        assert!(interpreter.thompson_vm("ab"));
        assert!(interpreter.thompson_vm("ba"));
        assert!(interpreter.thompson_vm(""));
        assert!(!interpreter.thompson_vm("c"));
        assert!(!interpreter.thompson_vm("abc"));
        assert!(!interpreter.thompson_vm("baac"));

        let interpreter = Interpreter::new(compile("(a+|b+)"));
        assert!(interpreter.thompson_vm("a"));
        assert!(interpreter.thompson_vm("b"));
        assert!(interpreter.thompson_vm("aaa"));
        assert!(interpreter.thompson_vm("bbb"));
        assert!(!interpreter.thompson_vm("ab"));
        assert!(!interpreter.thompson_vm("ba"));
        assert!(!interpreter.thompson_vm(""));
    }
}
