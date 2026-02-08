enum Expression {
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn evaluate(&self) -> f64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Add(left, right) => left.evaluate() + right.evaluate(),
            Expression::Subtract(left, right) => left.evaluate() - right.evaluate(),
            Expression::Multiply(left, right) => left.evaluate() * right.evaluate(),
            Expression::Divide(left, right) => left.evaluate() / right.evaluate(),
        }
    }
}

fn main() {
    let expr = Expression::Add(
    Box::new(Expression::Number(2.0)),
    Box::new(Expression::Multiply(
        Box::new(Expression::Number(3.0)),
        Box::new(Expression::Number(4.0))
        ))
    );
    println!("{}", expr.evaluate()); // 14.0
}