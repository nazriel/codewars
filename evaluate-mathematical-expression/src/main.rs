fn peek(tokens: &str, current: usize, pos: usize) -> char {
    let index = current + pos;
    if index < tokens.len() {
        tokens.chars().nth(index).unwrap()
    } else {
        ' '
    }
}

fn is_operator(ch: char) -> bool {
    ch == '+' || ch == '-' || ch == '*' || ch == '/'
}

fn is_digit(ch: char) -> bool {
    ch.is_digit(10) || ch == '.'
}

fn has_precedence(op1: char, op2: char) -> bool {
    if op2 == '(' || op2 == ')' {
        return false;
    }

    if (op1 == '*' || op1 == '/') && (op2 == '+' || op2 == '-') {
        return false;
    }

    return true;
}

fn evaluate_expression(input: &str) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();
    let mut ops: Vec<String> = Vec::new();
    let mut all: Vec<String> = Vec::new();

    for token in input.split_whitespace() {
        let mut num = String::new();
        for (pos, ch) in token.chars().enumerate() {
            if ch == ' ' {
                continue;
            }

            if ch == '#' {
                dbg!(&stack);
                dbg!(&ops);
                panic!("halt");
            }

            if is_digit(ch) {
                num.push(ch);
                all.push(ch.to_string());
            } else {
                if !num.is_empty() {
                    stack.push(num.parse::<f64>().unwrap());
                    num.clear();
                }

                if ch == '-' {
                    // dbg!("MAYBE!!!!");
                    if let Some(last) = all.last() {
                        if is_operator(last.chars().nth(0).unwrap()) {
                            num.push(ch);
                            all.push(ch.to_string());
                            continue;
                        }
                    } else if pos == 0 {
                        num.push(ch);
                        all.push(ch.to_string());
                        continue;
                    }
                }

                if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
                    while !ops.is_empty() && has_precedence(ch, ops.last())
                    {
                        values.push(applyOp(ops.pop(),
                                        values.pop(),
                                        values.pop()));
                    }


                    ops.push(ch.to_string());
                    all.push(ch.to_string());
                }
            }
        }
        if !num.is_empty() {
            stack.push(num.parse::<f64>().unwrap());
        }
    }

    dbg!(&stack);
    dbg!(&ops);

    for op in ops {
        let right = stack.pop().ok_or("Not implemented:".to_string())?;
        let left = stack.pop().ok_or("Not implemented".to_string())?;
        let op = op.as_str();

        let result = match op {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => return Err("Not implemented".to_string()),
        };

        stack.push(result);
    }

    stack.pop().ok_or("Not implemented".to_string())
}

fn calc(expr: &str) -> f64 {
    evaluate_expression(expr).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Wrap custom message to reduce repitition
    macro_rules! assert_expr_eq {
        ($expr: expr, $expect: expr) => {
            assert_eq!(
                calc($expr),
                $expect,
                "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
                $expr,
                $expect,
                calc($expr),
            );
        }
    }

    #[test]
    fn single_values() {
        assert_expr_eq!("0", 0.0);
        assert_expr_eq!("1", 1.0);
        assert_expr_eq!("42", 42.0);
        assert_expr_eq!("350", 350.0);
    }

    #[test]
    fn basic_operations() {
        assert_expr_eq!("1 + 1", 2.0);
        assert_expr_eq!("1 - 1", 0.0);
        assert_expr_eq!("1 * 1", 1.0);
        assert_expr_eq!("1 / 1", 1.0);
        assert_expr_eq!("12 * 123", 1476.0);
    }

    #[test]
    fn whitespace_between_operators_and_operands() {
        assert_expr_eq!("1-1", 0.0);
        assert_expr_eq!("2-1.2", 0.8);
        assert_expr_eq!("1 -1", 0.0);
        assert_expr_eq!("1- 1", 0.0);
        assert_expr_eq!("1* 1", 1.0);
    }

    #[test]
    fn unary_minuses() {
        assert_expr_eq!("1- -1", 2.0);
        assert_expr_eq!("1--1", 2.0);
        assert_expr_eq!("1 - -1", 2.0);
        assert_expr_eq!("-42", -42.0);
    }

    // #[test]
    // fn parentheses() {
    //     assert_expr_eq!("(1)", 1.0);
    //     assert_expr_eq!("((1))", 1.0);
    //     assert_expr_eq!("((80 - (19)))#", 61.0);
    // }


    #[test]
    fn priority() {
        assert_expr_eq!("1 + 2 * 3", 7.0);
    }
    // #[test]
    // fn multiple_operators() {
    //     assert_expr_eq!("12* 123/(-5 + 2)", -492.0);
    //     assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
    //     assert_expr_eq!("2 /2+3 * 4.75- -6", 21.25);
    //     assert_expr_eq!("2 / (2 + 3) * 4.33 - -6", 7.732);
    //     assert_expr_eq!("(1 - 2) + -(-(-(-4)))", 3.0);
    //     assert_expr_eq!("((2.33 / (2.9+3.5)*4) - -6)", 7.45625);
    // }
}
