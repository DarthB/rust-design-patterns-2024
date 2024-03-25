use rust_design_patterns::typestate::{
    build_distillation_column_as_typestate, ColumnWithEquations,
};

/// type-state pattern example with a distillation column that has three states.
///
/// 1. Configured (generate a valid distillation column with the help of the Builder)
/// 2. ReadyForSimulation (contains a set of equations that can be used for mathemtical simulation)
/// 3. Simulated (contains simulation results like the temperature or pressure profile)
///
/// As this is only for show-case the methods for simulation are simple placeholders. In fact only
/// linear functions are used to determine the temperature and pressure profile directly from the
/// starting pressure/temperature.
fn main() {
    // Add a debug dump of the distillation column in configured state.
    let column = build_distillation_column_as_typestate();
    println!("\nDUMP:\n{:?}\n", column);

    // Switch to ReadyForSimulation state and output the equations
    let column = column.generate_system_of_equations();
    println!(
        "Equations:\n{}\n",
        column.iter_equations().collect::<Vec<&str>>().join("\n")
    );

    // Switch to Simulated state and output the temperature + pressure profile
    let column = column.simulate();
    println!("Temperature Profile: {:?}", column.temperature_profile());
    println!("Pressure Profile: {:?}\n", column.pressure_profile());

    // Use change_parameters to get back to configured state and debug dump the column
    println!("\nChanging Input data...");
    let column = column
        .change_parameters(|b| {
            b.set_stages(40).set_d2f_ratio(0.5).set_reflux_ratio(2.2);
        })
        .unwrap();
    println!("DUMP:\n{:?}", column);
}
