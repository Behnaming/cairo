use std::fs;
use std::path::PathBuf;

use cairo_lang_sierra::extensions::builtin_cost::CostTokenType;
use cairo_lang_sierra::program::{Program, StatementIdx};
use test_case::test_case;

use crate::gas_info::GasInfo;
use crate::{calc_gas_info, CostError};

/// Returns a parsed example program from the example directory.
fn get_example_program(name: &str) -> Program {
    // Pop the "/sierra_gas" suffix.
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_owned();
    path.extend(["cairo-lang-sierra", "examples", &format!("{name}.sierra")].into_iter());
    cairo_lang_sierra::ProgramParser::new().parse(&fs::read_to_string(path).unwrap()).unwrap()
}

#[test_case("fib_jumps",
            Ok(GasInfo {
                variable_values: [
                    ((StatementIdx(40), CostTokenType::Step), 4),
                    ((StatementIdx(27), CostTokenType::Step), 8),
                    ((StatementIdx(49), CostTokenType::Step), 0),
                    ((StatementIdx(13), CostTokenType::Step), 11),
                    ((StatementIdx(3), CostTokenType::Step), 13),
                ].into_iter().collect(),
                function_costs: [(
                    "Fibonacci".into(),
                    [(CostTokenType::Step, 17)].into_iter().collect()
                )].into_iter().collect()
            });
            "fib_jumps")]
#[test_case("fib_recursive",
            Ok(GasInfo {
                variable_values: [
                    ((StatementIdx(27), CostTokenType::Step), 39),
                    ((StatementIdx(40), CostTokenType::Step), 0),
                    ((StatementIdx(49), CostTokenType::Step), 0),
                    ((StatementIdx(20), CostTokenType::Step), 4),
                    ((StatementIdx(9), CostTokenType::Step), 6),
                ].into_iter().collect(),
                function_costs: [(
                    "Fibonacci".into(),
                    [(CostTokenType::Step, 14)].into_iter().collect()
                )].into_iter().collect()
            });
            "fib_recursive")]
fn solve_gas(path: &str, expected: Result<GasInfo, CostError>) {
    pretty_assertions::assert_eq!(calc_gas_info(&get_example_program(path)), expected);
}
