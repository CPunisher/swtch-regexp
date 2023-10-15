pub struct Group(pub(super) Vec<Expr>);

pub struct Expr(pub(super) Vec<FactorConn>);

pub struct FactorConn(pub(super) Vec<Factor>);

pub enum Factor {
    Plain(Term),
    ZeroOrOne(Term),
    ZeroOrMore(Term),
    OneOrMore(Term),
}

pub enum Term {
    Char(char),
    Group(Group),
}
