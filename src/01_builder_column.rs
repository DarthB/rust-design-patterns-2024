use rust_design_patterns::builder::{DistillationColumn, DistillationColumnBuilder};

/// builder_column example
///
/// Shows how the `DistilationColumnBuilder` is used.
/// Generates a valid and an invalid column
fn main() {
    let sizeof_dc = std::mem::size_of::<DistillationColumn>();
    println!("Working with a structure of size {}", sizeof_dc);

    // use the builder to generate a valid column
    let mut builder: DistillationColumnBuilder = DistillationColumnBuilder::new();
    builder
        .set_stages(32)
        .set_d2f_ratio(0.8)
        .set_reflux_ratio(2.25)
        .set_temperature(85.)
        .set_pressure(1.)
        .add_feed_position(16)
        .add_feed_position(24);

    // build and reuse to acess the result
    let res_column = builder.build_and_reuse();
    // perform a runtime check if the column is valid and debug dump it contents
    match res_column {
        Ok(col) => println!("Column: {:?}", col),
        Err(e) => println!("Errors:\n{}", e),
    }

    // use the builder and generate another invalid column
    builder.clear_feed_positions().set_stages(2);
    // use the consume variant of build and perform a runtime check. Output the errors one per line
    let res_wrong_col = builder.build();
    match res_wrong_col {
        Ok(col) => println!("Column: {:?}", col),
        Err(e) => println!("Errors:\n{}", e),
    }

    // builder.add_feed_position(2); // If uncommented: Error as builder has been consumed
}
