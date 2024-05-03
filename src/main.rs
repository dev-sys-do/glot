use anyhow::anyhow;
use pest::{self, Parser};

pub type Result<T> = anyhow::Result<T>;

#[derive(pest_derive::Parser)]
#[grammar = "glot.pest"]
struct GlotParser;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Modulo,
}

impl TryFrom<Rule> for Operator {
    type Error = anyhow::Error;

    fn try_from(rule: Rule) -> Result<Self> {
        match rule {
            Rule::Minus => Ok(Operator::Substract),
            Rule::Plus => Ok(Operator::Add),
            Rule::Mult => Ok(Operator::Multiply),
            _ => Err(anyhow!("Invalid operator rule {:?}", rule)),
        }
    }
}

impl Operator {
    fn apply_unary(&self, term: i32) -> Result<i32> {
        match self {
            Operator::Add => Ok(term),
            Operator::Substract => Ok(-term),
            _ => Err(anyhow!("Invalid unary operator {:?}", self)),
        }
    }

    fn apply_binary(&self, l: i32, r: i32) -> Result<i32> {
        match self {
            Operator::Add => Ok(l + r),
            Operator::Substract => Ok(l - r),
            Operator::Multiply => Ok(l * r),
            Operator::Divide => Ok(l / r),
            Operator::Modulo => Ok(l % r),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Integer(i32),
    UnaryExpr {
        op: Operator,
        term: Box<Node>,
    },
    BinaryExpr {
        l_term: Box<Node>,
        op: Operator,
        r_term: Box<Node>,
    },
}

impl Node {
    fn new_from_integer(tokens: pest::iterators::Pair<Rule>) -> Result<Node> {
        Ok(Node::Integer(tokens.as_str().parse().map_err(|e| {
            anyhow!("{:?} Invalid integer expression {:?}", e, tokens.as_str())
        })?))
    }

    fn new_from_term(tokens: pest::iterators::Pair<Rule>) -> Result<Node> {
        match tokens.as_rule() {
            Rule::Integer => Node::new_from_integer(tokens),
            Rule::Expr => {
                let inner_tokens = tokens
                    .clone()
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("Invalid tokens {:?}", tokens.as_str()))?;

                Node::new_from_expr(inner_tokens)
            }
            _ => Err(anyhow!("Unknown TERM expr: {:?}", tokens.as_str())),
        }
    }

    fn new_from_unary(tokens: pest::iterators::Pair<Rule>) -> Result<Node> {
        let mut inner_tokens = tokens.clone().into_inner();
        let operator = inner_tokens
            .next()
            .ok_or(anyhow!("Invalid tokens {:?}", tokens.as_str()))?;
        let term = inner_tokens
            .next()
            .ok_or(anyhow!("Invalid tokens {:?}", tokens.as_str()))?;

        let op = Operator::try_from(operator.as_rule())?;
        let term = Box::new(Node::new_from_term(term)?);

        Ok(Node::UnaryExpr { op, term })
    }

    fn new_from_binary(tokens: pest::iterators::Pair<Rule>) -> Result<Node> {
        let mut inner_tokens = tokens.clone().into_inner();
        let lhs = inner_tokens
            .next()
            .ok_or(anyhow!("Invalid tokens {:?}", tokens.as_str()))?;
        let operator = inner_tokens
            .next()
            .ok_or(anyhow!("Invalid tokens {:?}", tokens.as_str()))?;
        let rhs = inner_tokens
            .next()
            .ok_or(anyhow!("Invalid tokens {:?}", tokens.as_str()))?;

        let op = Operator::try_from(operator.as_rule())?;
        let l_term = Box::new(Node::new_from_term(lhs)?);
        let r_term = Box::new(Node::new_from_term(rhs)?);

        Ok(Node::BinaryExpr { l_term, op, r_term })
    }

    fn new_from_expr(tokens: pest::iterators::Pair<Rule>) -> Result<Node> {
        let inner_tokens = tokens
            .clone()
            .into_inner()
            .next()
            .ok_or(anyhow!("Invalid tokens {:?}", tokens.as_str()))?;

        match tokens.as_rule() {
            Rule::Expr => Node::new_from_expr(inner_tokens),
            Rule::BinaryExpr => Node::new_from_binary(tokens),
            Rule::UnaryExpr => Node::new_from_unary(tokens),
            _ => Err(anyhow!("Unknown expr: {:?}", tokens.as_rule())),
        }
    }

    fn interpret(&self) -> Result<i32> {
        match self {
            Node::Integer(i) => Ok(*i),
            Node::UnaryExpr { op, term } => op.apply_unary(term.interpret()?),
            Node::BinaryExpr { l_term, op, r_term } => {
                op.apply_binary(l_term.interpret()?, r_term.interpret()?)
            }
        }
    }
}

pub fn parse(source: &str) -> Result<Vec<Node>> {
    let mut ast = vec![];
    let pairs = GlotParser::parse(Rule::Program, source)
        .map_err(|e| anyhow!("Failed to parse \"{}\" {:?}", source, e))?;

    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(Node::new_from_expr(pair)?);
        }
    }

    Ok(ast)
}

fn main() -> Result<()> {
    let ast = parse("(2*9)-1+(1+8)")?;
    for a in ast {
        println!("AST {:?}  {}", a, a.interpret()?);
    }

    Ok(())
}
