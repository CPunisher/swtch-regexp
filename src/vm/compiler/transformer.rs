use std::{collections::HashMap, vec};

use crate::vm::inst::{Inst, InstBlock, InstNode};

use super::ast::{self, Factor};

#[derive(Default)]
pub struct Transformer {
    nodes: Vec<InstNode>,
}

impl Transformer {
    fn add_node(&mut self, node: InstNode) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }
}

impl Transformer {
    fn transform_group(&mut self, ast: ast::Group) -> usize {
        let mut e_list = vec![];
        for expr in ast.0 {
            let id = self.transform_expr(expr);
            e_list.push(InstBlock::InstNodeIndex(id));
        }
        self.add_node(InstNode(e_list))
    }

    fn transform_expr(&mut self, ast: ast::Expr) -> usize {
        let mut e_list = vec![];
        for factor_conn in ast.0 {
            let id = self.transform_factor_conn(factor_conn);
            e_list.push(id);
        }

        let mut split_ids = vec![];
        let mut l_list = vec![];
        let noop = self.add_node(InstNode::single_inst(Inst::Noop));
        for branch in e_list {
            let branch = InstBlock::InstNodeIndex(branch);
            let jmp = InstBlock::Inst(Inst::Jump(noop));
            let l = InstNode(vec![branch, jmp]);
            let l_id = self.add_node(l);
            split_ids.push(l_id);
            l_list.push(InstBlock::InstNodeIndex(l_id));
        }
        l_list.push(InstBlock::InstNodeIndex(noop));
        if split_ids.len() > 1 {
            l_list.insert(0, InstBlock::Inst(Inst::Split(split_ids)));
        }
        self.add_node(InstNode(l_list))
    }

    fn transform_factor_conn(&mut self, ast: ast::FactorConn) -> usize {
        let mut e_list = vec![];
        for factor in ast.0 {
            let id = self.transform_factor(factor);
            e_list.push(InstBlock::InstNodeIndex(id));
        }
        self.add_node(InstNode(e_list))
    }

    fn transform_factor(&mut self, ast: ast::Factor) -> usize {
        match ast {
            Factor::Plain(term) => self.transform_term(term),
            Factor::ZeroOrOne(term) => {
                let l1 = self.transform_term(term);
                let l2 = self.add_node(InstNode::single_inst(Inst::Noop));
                self.add_node(InstNode(vec![
                    InstBlock::Inst(Inst::Split(vec![l1, l2])),
                    InstBlock::InstNodeIndex(l1),
                    InstBlock::InstNodeIndex(l2),
                ]))
            }
            Factor::ZeroOrMore(term) => {
                let e = self.transform_term(term);
                let l3 = self.add_node(InstNode::single_inst(Inst::Noop));
                let l1 = self.add_node(InstNode::single_inst(Inst::Split(vec![l3 + 2, l3])));
                let l2 = self.add_node(InstNode(vec![
                    InstBlock::InstNodeIndex(e),
                    InstBlock::Inst(Inst::Jump(l1)),
                ]));
                self.add_node(InstNode(vec![
                    InstBlock::InstNodeIndex(l1),
                    InstBlock::InstNodeIndex(l2),
                    InstBlock::InstNodeIndex(l3),
                ]))
            }
            Factor::OneOrMore(term) => {
                let e = self.transform_term(term);
                let l3 = self.add_node(InstNode::single_inst(Inst::Noop));
                let l1 = self.add_node(InstNode(vec![
                    InstBlock::InstNodeIndex(e),
                    InstBlock::Inst(Inst::Split(vec![l3 + 1, l3])),
                ]));
                self.add_node(InstNode(vec![
                    InstBlock::InstNodeIndex(l1),
                    InstBlock::InstNodeIndex(l3),
                ]))
            }
        }
    }

    fn transform_term(&mut self, ast: ast::Term) -> usize {
        match ast {
            ast::Term::Char(c) => self.add_node(InstNode::single_inst(Inst::Char(c))),
            ast::Term::Group(group) => self.transform_group(group),
        }
    }

    pub fn transform(&mut self, ast: ast::Group) -> Vec<Inst> {
        let start = self.transform_group(ast);
        let mut generator = InstructGenerator {
            inst_list: vec![],
            inst_mapping: HashMap::new(),
        };
        generator.dfs(start, &self.nodes);
        // Remap instruction targets
        for inst in generator.inst_list.iter_mut() {
            match inst {
                Inst::Split(ids) => {
                    let mut new_ids = vec![];
                    for id in ids.iter() {
                        new_ids.push(*generator.inst_mapping.get(id).unwrap());
                    }
                    *ids = new_ids;
                }
                Inst::Jump(id) => {
                    *id = *generator.inst_mapping.get(id).unwrap();
                }
                _ => {}
            }
        }
        generator.inst_list.push(Inst::Match);
        generator.inst_list
    }
}

struct InstructGenerator {
    inst_list: Vec<Inst>,
    inst_mapping: HashMap<usize, usize>,
}

impl InstructGenerator {
    fn dfs(&mut self, id: usize, nodes: &Vec<InstNode>) {
        if self.inst_mapping.contains_key(&id) {
            return;
        }

        for (index, block) in nodes[id].0.iter().enumerate() {
            if index == 0 {
                self.inst_mapping.insert(id, self.inst_list.len());
            }
            match block {
                InstBlock::Inst(inst) => {
                    if matches!(inst, Inst::Noop) {
                        continue;
                    }
                    self.inst_list.push(inst.clone());
                }
                InstBlock::InstNodeIndex(id) => self.dfs(*id, nodes),
            }
        }
    }
}
