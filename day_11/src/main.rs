use nom::character::complete::{self as cc, one_of, space1};

use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    IResult,
};

use nom::error::ParseError;
use nom_locate::LocatedSpan;
use nom_supreme::error::{BaseErrorKind, ErrorTree, GenericErrorTree};
use nom_supreme::final_parser::final_parser;

use miette::GraphicalReportHandler;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items_inspected: u64,
    pub items: Vec<u64>,
    pub operation: Operation,
    pub divisor: u64,
    pub receiving_monkey_if_true: usize,
    pub receiving_monkey_if_false: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Add(Term, Term),
    Multiply(Term, Term),
}

impl Operation {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Add(lhs, rhs) => lhs.eval(old) + rhs.eval(old),
            Operation::Multiply(lhs, rhs) => lhs.eval(old) * rhs.eval(old),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Term {
    Old,
    Constant(u64),
}

impl Term {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Term::Old => old,
            Term::Constant(c) => c,
        }
    }
}

pub fn parse_term<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Term, E> {
    alt((value(Term::Old, tag("old")), map(cc::u64, Term::Constant)))(i)
}

pub fn parse_operation<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Operation, E> {
    let (i, (lhs, op, rhs)) = preceded(
        tag("new = "),
        tuple((
            parse_term,
            preceded(space1, one_of("*+")),
            preceded(space1, parse_term),
        )),
    )(i)?;
    let op = match op {
        '*' => Operation::Multiply(lhs, rhs),
        '+' => Operation::Add(lhs, rhs),
        _ => unreachable!(),
    };

    Ok((i, op))
}

pub fn parse_monkey<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Monkey, E> {
    // sample input:
    // Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3

    let (i, _) = tuple((tag("Monkey "), cc::u64, tag(":\n")))(i)?;

    let (i, (_, _, items, _)) = tuple((
        space1,
        tag("Starting items: "),
        separated_list1(tag(", "), cc::u64),
        tag("\n"),
    ))(i)?;

    let (i, (_, _, operation, _)) =
        tuple((space1, tag("Operation: "), parse_operation, tag("\n")))(i)?;

    let (i, (_, _, divisor, _)) =
        tuple((space1, tag("Test: divisible by "), cc::u64, tag("\n")))(i)?;

    let (i, (_, _, receiving_monkey_if_true, _)) = tuple((
        space1,
        tag("If true: throw to monkey "),
        map(cc::u64, |x| x as usize),
        tag("\n"),
    ))(i)?;

    let (i, (_, _, receiving_monkey_if_false, _)) = tuple((
        space1,
        tag("If false: throw to monkey "),
        map(cc::u64, |x| x as usize),
        tag("\n"),
    ))(i)?;

    Ok((
        i,
        Monkey {
            items_inspected: 0,
            items,
            operation,
            divisor,
            receiving_monkey_if_true,
            receiving_monkey_if_false,
        },
    ))
}

pub fn parse_all_monkeys<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<Monkey>, E> {
    separated_list1(tag("\n"), parse_monkey)(i)
}

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("bad input")]
struct BadInput {
    #[source_code]
    src: &'static str,

    #[label("{kind}")]
    bad_part: miette::SourceSpan,

    kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
}

fn main() {
    let input_static = concat!(include_str!("../input-sample.txt"), "\n");
    let input = Span::new(input_static);

    let monkeys_res: Result<_, ErrorTree<Span>> =
        final_parser(parse_all_monkeys::<ErrorTree<Span>>)(input);

    let monkeys = match monkeys_res {
        Ok(monkeys) => monkeys,
        Err(e) => {
            match e {
                GenericErrorTree::Base { location, kind } => {
                    let offset = location.location_offset().into();
                    let err = BadInput {
                        src: input_static,
                        bad_part: miette::SourceSpan::new(offset, 0.into()),
                        kind,
                    };
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .render_report(&mut s, &err)
                        .unwrap();
                    println!("{s}");
                }
                GenericErrorTree::Stack { .. } => todo!("stack error"),
                GenericErrorTree::Alt(_i) => {
                    println!("{:?}", _i);
                    todo!("alt error");
                }
            }
            return;
        }
    };

    for m in &monkeys {
        println!("{:?}", m);
    }
}
