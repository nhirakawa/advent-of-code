use anyhow::bail;
use itertools::Itertools;
use log::debug;
use model::*;
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let (inputs, expressions) = parse::parse(input)?;

    let values = evaluate(&inputs, &expressions)?;

    let mut z_values = Vec::new();

    for (wire, value) in values {
        if wire.starts_with_z() {
            z_values.push((wire, value));
        }
    }

    z_values.sort_by_key(|(wire, _)| wire.to_string());
    z_values.reverse();

    let mut result = String::new();

    for (_, value) in z_values {
        result.push_str(&value.as_u8().to_string());
    }

    u64::from_str_radix(&result, 2)
        .map(|u| u.to_string())
        .map_err(Into::into)
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let (_inputs, expressions) = parse::parse(input)?;

    output::write_dot(&expressions)?;

    let mut swapped_wires = Vec::new();

    for i in 1..45 {
        swapped_wires.extend(check_bit_gates(i, &expressions)?);
    }

    let swapped_wires = swapped_wires
        .into_iter()
        .map(|wire| wire.to_string())
        .sorted()
        .join(",");

    Ok(swapped_wires)
}

fn check_bit_gates(bit: u8, expressions: &[Expression]) -> anyhow::Result<HashSet<Wire>> {
    println!("Checking bit {}", bit);

    let mut wires = HashSet::new();

    // Exploration shows that this should never fail
    let xor_n = find_xor_gate_with_x_y_inputs(bit, expressions)?;

    // check that the result of xor_n is an input to an AND gate
    let and = expressions
        .iter()
        .filter(|expression| expression.has_and_with_input(&xor_n.result()))
        .collect_vec();

    if and.len() > 1 {
        bail!(
            "Expected 1 AND expression with input {}, found {}: {:?}",
            xor_n.result(),
            and.len(),
            and
        );
    }

    if and.is_empty() {
        println!("No AND gate found for XOR({}): {}", bit, xor_n.result());
        wires.insert(xor_n.result());
    } else {
        let and = and[0];

        let has_or_input = expressions
            .iter()
            .any(|expression| expression.has_or_with_input(&and.result()));

        if !has_or_input {
            println!("AND result {} is not an input to an OR gate", and.result());
            wires.insert(and.result());
        }
    }

    let xor = expressions
        .iter()
        .filter(|expression| expression.has_xor_with_input(&xor_n.result()))
        .collect_vec();

    if xor.len() > 1 {
        bail!(
            "Expected 1 XOR expression with input {}, found {}: {:?}",
            xor_n.result(),
            xor.len(),
            xor
        );
    }

    if xor.is_empty() {
        println!("No XOR gate found for XOR({}): {}", bit, xor_n.result());
        wires.insert(xor_n.result());
    } else {
        let xor = xor[0];

        if !xor.result().starts_with_z() {
            println!("XOR result {} is not a Z wire", xor.result());
            wires.insert(xor.result());
        }
    }

    // Exploration shows that this should never fail
    let and_n = find_and_gate_with_x_y_inputs(bit, expressions)?;

    let or = expressions
        .iter()
        .filter(|expression| expression.has_or_with_input(&and_n.result()))
        .collect_vec();

    if or.len() > 1 {
        bail!(
            "Expected 0-or-1 OR expression with input {}, found {}: {:?}",
            and_n.result(),
            or.len(),
            or
        );
    }

    if or.is_empty() {
        println!("No OR gate found for AND({}): {}", bit, and_n.result());
        wires.insert(and_n.result());
    } else {
        let or = or[0];

        if !(or.result().starts_with_z() && or.result().as_u8() == Some(45)) {
            let has_and_input = expressions
                .iter()
                .any(|expression| expression.has_and_with_input(&or.result()));

            let has_xor_input = expressions
                .iter()
                .any(|expression| expression.has_xor_with_input(&or.result()));

            if !has_and_input || !has_xor_input {
                println!("OR result {} is not an AND or XOR gate", or.result());
                wires.insert(or.result());
            }
        }
    }

    Ok(wires)
}

fn find_xor_gate_with_x_y_inputs(
    bit: u8,
    expressions: &[Expression],
) -> anyhow::Result<Expression> {
    let xor_n = expressions
        .iter()
        .filter(|expression| {
            expression.has_xor_with_input(&Wire::InputX(bit))
                && expression.has_xor_with_input(&Wire::InputY(bit))
        })
        .collect_vec();

    if xor_n.len() != 1 {
        bail!(
            "Expected 1 XOR_N expression for bit {bit}, found {}: {:?}",
            xor_n.len(),
            xor_n
        );
    }

    Ok(*xor_n[0])
}

fn find_and_gate_with_x_y_inputs(
    bit: u8,
    expressions: &[Expression],
) -> anyhow::Result<Expression> {
    let and_n = expressions
        .iter()
        .filter(|expression| {
            expression.has_and_with_input(&Wire::InputX(bit))
                && expression.has_and_with_input(&Wire::InputY(bit))
        })
        .collect_vec();

    if and_n.len() != 1 {
        bail!(
            "Expected 1 AND_N expression for bit {bit}, found {}: {:?}",
            and_n.len(),
            and_n
        );
    }

    Ok(*and_n[0])
}

fn evaluate(
    inputs: &[(Wire, Value)],
    expressions: &[Expression],
) -> anyhow::Result<HashMap<Wire, Value>> {
    let mut values = HashMap::new();

    for (wire, value) in inputs {
        values.insert(*wire, *value);
    }

    loop {
        let mut all_results_have_values = true;
        for expression in expressions {
            if !values.contains_key(&expression.result()) {
                all_results_have_values = false;
                break;
            }
        }

        if all_results_have_values {
            break;
        }

        for expression in expressions {
            if values.contains_key(&expression.result()) {
                continue;
            }

            if !values.contains_key(&expression.left_operand()) {
                continue;
            }

            if !values.contains_key(&expression.right_operand()) {
                continue;
            }

            let left_operand = values.get(&expression.left_operand()).ok_or_else(|| {
                anyhow::anyhow!("Missing value for wire {}", expression.left_operand())
            })?;

            let right_operand = values.get(&expression.right_operand()).ok_or_else(|| {
                anyhow::anyhow!("Missing value for wire {}", expression.right_operand())
            })?;

            let result = match expression.gate() {
                Gate::And => left_operand & right_operand,
                Gate::Or => left_operand | right_operand,
                Gate::Xor => left_operand ^ right_operand,
            };

            debug!(
                "Evaluating expression: {} {} {} -> {} = {}",
                expression.left_operand(),
                match expression.gate() {
                    Gate::And => "AND",
                    Gate::Or => "OR",
                    Gate::Xor => "XOR",
                },
                expression.right_operand(),
                expression.result(),
                result.as_u8()
            );

            values.insert(expression.result(), result);
        }
    }

    Ok(values)
}

mod output {
    use crate::common::base::{Day, Year};
    use crate::common::debug::{self, DotConfig, LayoutEngine, OutputFormat};

    use super::{Expression, Gate};

    pub fn write_dot(expressions: &[Expression]) -> anyhow::Result<()> {
        let mut dot = String::new();

        dot.push_str("digraph G {\n");

        for expression in expressions.iter() {
            let gate_name = format!(
                "{}_{}_{}",
                expression.left_operand(),
                expression.gate(),
                expression.right_operand()
            );

            let gate_shape = match expression.gate() {
                Gate::And => "ellipse",
                Gate::Or => "parallelogram",
                Gate::Xor => "diamond",
            };

            dot.push_str(&format!(
                "{} [label=\"{}\" shape=\"{}\"];\n",
                gate_name,
                expression.gate(),
                gate_shape
            ));

            dot.push_str(&format!(
                "\t{} -> {};\n",
                expression.left_operand(),
                gate_name,
            ));

            dot.push_str(&format!(
                "\t{} -> {};\n",
                expression.right_operand(),
                gate_name
            ));

            dot.push_str(&format!("\t{} -> {};\n", gate_name, expression.result()));
        }

        dot.push_str("}\n");

        let config = DotConfig {
            layout_engine: LayoutEngine::Dot,
            output_format: OutputFormat::Svg,
        };

        let writer = debug::OutputWriter::new(Year::Year2024, Day::Day24);

        writer.write_dot("graph", &dot, config)
    }
}

mod model {
    use std::{
        fmt::Display,
        ops::{BitAnd, BitOr, BitXor},
    };

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
    pub enum Value {
        Zero,
        One,
    }

    impl Value {
        pub fn as_u8(&self) -> u8 {
            match self {
                Value::Zero => 0,
                Value::One => 1,
            }
        }

        pub fn as_bool(&self) -> bool {
            match self {
                Value::Zero => false,
                Value::One => true,
            }
        }
    }

    impl BitAnd for &Value {
        type Output = Value;

        fn bitand(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Value::Zero, Value::Zero) => Value::Zero,
                (Value::Zero, Value::One) => Value::Zero,
                (Value::One, Value::Zero) => Value::Zero,
                (Value::One, Value::One) => Value::One,
            }
        }
    }

    impl BitOr for &Value {
        type Output = Value;

        fn bitor(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Value::Zero, Value::Zero) => Value::Zero,
                (Value::Zero, Value::One) => Value::One,
                (Value::One, Value::Zero) => Value::One,
                (Value::One, Value::One) => Value::One,
            }
        }
    }

    impl BitXor for &Value {
        type Output = Value;

        fn bitxor(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Value::Zero, Value::Zero) => Value::Zero,
                (Value::Zero, Value::One) => Value::One,
                (Value::One, Value::Zero) => Value::One,
                (Value::One, Value::One) => Value::Zero,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum Wire {
        Literal([char; 3]),
        InputX(u8),
        InputY(u8),
    }

    impl Wire {
        pub fn starts_with_z(&self) -> bool {
            matches!(self, Self::Literal(['z', _, _,]))
        }

        pub fn as_u8(&self) -> Option<u8> {
            match self {
                Self::Literal([_, b, c]) => {
                    if !b.is_ascii_digit() || !c.is_ascii_digit() {
                        return None;
                    }

                    let b = b.to_digit(10).unwrap();
                    let c = c.to_digit(10).unwrap();
                    format!("{}{}", b, c).parse().ok()
                }
                Self::InputX(u) => Some(*u),
                Self::InputY(u) => Some(*u),
            }
        }
    }

    impl Display for Wire {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Wire::Literal(name) => {
                    let [a, b, c] = name;
                    write!(f, "{}{}{}", a, b, c)
                }
                Wire::InputX(u) => write!(f, "x{}", u),
                Wire::InputY(u) => write!(f, "y{}", u),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum Gate {
        And,
        Or,
        Xor,
    }

    impl Display for Gate {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Gate::And => write!(f, "AND"),
                Gate::Or => write!(f, "OR"),
                Gate::Xor => write!(f, "XOR"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    pub struct Expression {
        left_operand: Wire,
        gate: Gate,
        right_operand: Wire,
        result: Wire,
    }

    impl Expression {
        pub fn new(left_operand: Wire, gate: Gate, right_operand: Wire, result: Wire) -> Self {
            Self {
                left_operand,
                gate,
                right_operand,
                result,
            }
        }

        pub fn left_operand(&self) -> Wire {
            self.left_operand
        }

        pub fn gate(&self) -> Gate {
            self.gate
        }

        pub fn right_operand(&self) -> Wire {
            self.right_operand
        }

        pub fn result(&self) -> Wire {
            self.result
        }

        pub fn has_xor_with_input(&self, wire: &Wire) -> bool {
            self.gate == Gate::Xor && (&self.left_operand == wire || &self.right_operand == wire)
        }

        pub fn has_and_with_input(&self, wire: &Wire) -> bool {
            self.gate == Gate::And && (&self.left_operand == wire || &self.right_operand == wire)
        }

        pub fn has_or_with_input(&self, wire: &Wire) -> bool {
            self.gate == Gate::Or && (&self.left_operand == wire || &self.right_operand == wire)
        }
    }

    impl From<(Wire, Gate, Wire, Wire)> for Expression {
        fn from((left_operand, gate, right_operand, result): (Wire, Gate, Wire, Wire)) -> Self {
            Self::new(left_operand, gate, right_operand, result)
        }
    }

    impl Display for Expression {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {} {} -> {}",
                self.left_operand, self.gate, self.right_operand, self.result
            )
        }
    }
}

mod parse {
    use crate::common::parse::finish;

    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
        IResult, Parser,
    };

    #[allow(clippy::type_complexity)]
    pub fn parse(i: &str) -> anyhow::Result<(Vec<(Wire, Value)>, Vec<Expression>)> {
        finish(inputs_and_expressions, i)
    }

    #[allow(clippy::type_complexity)]
    fn inputs_and_expressions(i: &str) -> IResult<&str, (Vec<(Wire, Value)>, Vec<Expression>)> {
        separated_pair(inputs, tag("\n\n"), expressions).parse(i)
    }

    fn inputs(i: &str) -> IResult<&str, Vec<(Wire, Value)>> {
        separated_list1(tag("\n"), input).parse(i)
    }

    fn input(i: &str) -> IResult<&str, (Wire, Value)> {
        separated_pair(wire, tag(": "), value).parse(i)
    }

    fn value(i: &str) -> IResult<&str, Value> {
        let zero = map(tag("0"), |_| Value::Zero);
        let one = map(tag("1"), |_| Value::One);

        alt((zero, one)).parse(i)
    }

    fn expressions(i: &str) -> IResult<&str, Vec<Expression>> {
        separated_list1(tag("\n"), expression).parse(i)
    }

    fn expression(i: &str) -> IResult<&str, Expression> {
        map(
            (
                wire,
                tag(" "),
                alt((and_gate, or_gate, xor_gate)),
                tag(" "),
                wire,
                tag(" -> "),
                wire,
            ),
            |(left_operand, _, gate, _, right_operand, _, result)| {
                Expression::new(left_operand, gate, right_operand, result)
            },
        )
        .parse(i)
    }

    fn wire(i: &str) -> IResult<&str, Wire> {
        alt((wire_input_x, wire_input_y, wire_literal)).parse(i)
    }

    fn wire_input_x(i: &str) -> IResult<&str, Wire> {
        map((tag("x"), take(2usize)), |(_, digits): (&str, &str)| {
            let digit = digits.parse().unwrap();
            Wire::InputX(digit)
        })
        .parse(i)
    }

    fn wire_input_y(i: &str) -> IResult<&str, Wire> {
        map((tag("y"), take(2usize)), |(_, digits): (&str, &str)| {
            let digit = digits.parse().unwrap();
            Wire::InputY(digit)
        })
        .parse(i)
    }

    fn wire_literal(i: &str) -> IResult<&str, Wire> {
        map(take(3usize), |name: &str| {
            let mut chars = ['\0'; 3];
            for (i, c) in name.chars().enumerate() {
                chars[i] = c;
            }
            Wire::Literal(chars)
        })
        .parse(i)
    }

    fn and_gate(i: &str) -> IResult<&str, Gate> {
        map(tag("AND"), |_| Gate::And).parse(i)
    }

    fn or_gate(i: &str) -> IResult<&str, Gate> {
        map(tag("OR"), |_| Gate::Or).parse(i)
    }

    fn xor_gate(i: &str) -> IResult<&str, Gate> {
        map(tag("XOR"), |_| Gate::Xor).parse(i)
    }
}
