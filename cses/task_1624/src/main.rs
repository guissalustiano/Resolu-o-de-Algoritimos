fn n_queens(n: i32) -> uint {
    return n_queens_helper((1 << n as uint) - 1, 0, 0, 0);
}

fn n_queens_helper(all_ones: i32, left_diags: i32, columns: i32, right_diags: i32) -> uint {
    let mut solutions = 0;

    // We get valid_spots with some bit trickery. Effectively, each
    // of the parameters can be ORed together to create
    // an integer with all the conflicts together, which we then
    // invert and limit by ANDing with all_ones, our special value from
    // earlier.
    let mut valid_spots = !(left_diags | columns | right_diags) & all_ones;

    // Since valid_spots contains 1s in all of the locations that
    // are conflict-free, we know we have gone through all of
    // those locations when valid_spots is all 0s, i.e. when it is
    // 0.
    while valid_spots != 0 {
        // This is just bit trickery. For reasons involving the weird
        // behavior of two's complement integers, this creates an integer
        // which is all 0s except for a single 1 in the position of the
        // LSB of valid_spots.
        let spot = -valid_spots & valid_spots;

        // We then XOR that integer with the valid_spots to flip it to 0
        // in valid_spots.
        valid_spots = valid_spots ^ spot;

        // Make a recursive call. This is where we infer the conflicts
        // for the next row.
        solutions += n_queens_helper(
            all_ones,
            // We add a conflict in the current spot and then shift left,
            // which has the desired effect of moving all of the conflicts
            // that are created by left diagonals to the left one square.
            (left_diags | spot) << 1,
            // For columns we simply mark this column as filled by ORing
            // in the currentSpot.
            (columns | spot),
            // This is the same as the leftDiag shift, except we shift
            // right because these conflicts are caused by right
            // diagonals.
            (right_diags | spot) >> 1,
        );
    }

    // If columns is all blocked (i.e. if it is all ones) then we
    // have arrived at a solution because we have placed n queens.
    return solutions + ((columns == all_ones) as uint);
}
