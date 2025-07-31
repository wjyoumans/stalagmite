Comparison of two integer polynomial implementations and their performance under addition/summation.

### flint-style
IntPoly is a Vec of coefficients and a length, indicating the last non-zero coefficient. The coefficient vector may contain trailing zeros. After operations, the Vec is "normalized", which just updates the `length` value and doesn't drop anything.
    - pros: should mean less modifications to the coefficient vector overall, potentially less allocations.
    - cons: additional length `usize` parameter for each `IntPoly`, slightly more complex logic to account for differences in polynomial length and vector size. Also likely to be more memory-intensive since polynomials will not drop trailing zeros.

### rust-style
IntPoly is just a Vec. After operations, the Vec is normalized which potentially calls `Vec::truncate` and drops the remaining trailing zeros.
    - pros: simpler logic.
    - cons: If a polynomial is re-used many times (re: `std::iter::Sum`) there may be many dropped trailing zeros, and if added to a higher degree polynomial they will need to reallocate a `malachite::Integer`. I.e. dropping is cheap but potentially more expensive if we constantly reallocate. This is hopefully accounted for in the benchmarks by using varying sized polynomials.


### Result:
The rust style seems superior in almost ever test.

