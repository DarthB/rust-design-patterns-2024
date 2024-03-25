use crate::builder::{DistillationColumn, DistillationColumnBuilder};

#[derive(Debug)]
pub struct TypeStateColumn<S: ColumnState> {
    data: DistillationColumn,
    marker: std::marker::PhantomData<S>,
}

#[derive(Debug)]
pub struct Configured {}

#[derive(Debug)]
pub struct ReadyForSimulation {}

#[derive(Debug)]
pub struct Simulated {}

pub trait ColumnState {}
impl ColumnState for Configured {}
impl ColumnState for ReadyForSimulation {}
impl ColumnState for Simulated {}

pub trait ColumnWithEquations {
    fn equations(&self) -> &[String];

    fn iter_equations(&self) -> impl Iterator<Item = &str> + '_ {
        self.equations().iter().map(|s| s.as_str())
    }
}

impl ColumnWithEquations for TypeStateColumn<ReadyForSimulation> {
    fn equations(&self) -> &[String] {
        self.data.equations.as_ref()
    }
}
impl ColumnWithEquations for TypeStateColumn<Simulated> {
    fn equations(&self) -> &[String] {
        &self.data.equations.as_ref()
    }
}

impl<S: ColumnState> TypeStateColumn<S> {
    pub fn change_parameters<F: FnOnce(&mut DistillationColumnBuilder)>(
        self,
        closure: F,
    ) -> Result<TypeStateColumn<Configured>, String> {
        //...

        let mut builder = DistillationColumnBuilder::adapt(self.data);
        closure(&mut builder);
        let mut new_data = builder.build()?;
        new_data.equations.clear();
        new_data.temperature_profile.clear();
        new_data.pressure_profile.clear();
        Ok(TypeStateColumn::<Configured> {
            data: new_data,
            marker: std::marker::PhantomData,
        })
    }
}

impl TypeStateColumn<Configured> {
    pub fn generate_system_of_equations(mut self) -> TypeStateColumn<ReadyForSimulation> {
        // code that generates system of equations ...
        self.data.equations.push("m1+m2=m3+m4".to_owned());

        // change the state on return
        let reval = TypeStateColumn::<ReadyForSimulation> {
            data: self.data,
            marker: std::marker::PhantomData {},
        };
        reval
    }
}

impl TypeStateColumn<ReadyForSimulation> {
    pub fn simulate(mut self) -> TypeStateColumn<Simulated> {
        // simulation code ...
        let range = 0..self.data.stages;
        // T[s] = T0 + s*0.5
        self.data.temperature_profile = range
            .clone()
            .into_iter()
            .map(|s| self.data.temperature - s as f32 * 0.5)
            .collect();
        // P[s] = P0 - 0.01 * s
        self.data.pressure_profile = range
            .into_iter()
            .map(|s| self.data.pressure - s as f32 * 0.01)
            .collect();

        // change the state on return
        TypeStateColumn::<Simulated> {
            data: self.data,
            marker: std::marker::PhantomData,
        }
    }
}

impl TypeStateColumn<Simulated> {
    pub fn temperature_profile<'a>(&'a self) -> &[f32] {
        self.data.temperature_profile.as_ref()
    }

    pub fn pressure_profile<'a>(&'a self) -> &[f32] {
        &self.data.pressure_profile.as_ref()
    }
}

pub fn build_distillation_column_as_typestate() -> TypeStateColumn<Configured> {
    let mut builder = DistillationColumnBuilder::new();
    builder
        .set_stages(32)
        .set_d2f_ratio(0.8)
        .set_reflux_ratio(2.25)
        .set_temperature(95.)
        .set_pressure(1.)
        .add_feed_position(16)
        .add_feed_position(24);

    let res_column = builder.build_and_reuse().unwrap();

    TypeStateColumn::<Configured> {
        data: res_column,
        marker: std::marker::PhantomData,
    }
}
