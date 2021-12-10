//! How many paths from the top-left to bottom-right of a 20-by-20 grid?
use crate::utils;

fn solve_for(n: u64) -> u64 {
    // well-known fact (e.x. associate grid intersections with pascal's triangle)
    num::integer::binomial(2 * n, n)
}

utils::test::example!(2 => 6);
utils::test::problem!(u64: 20 => 137_846_528_820);
