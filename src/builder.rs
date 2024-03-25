#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Mixture(pub f32, pub f32, pub f32); // Water, Ethanol, Benzene

/// Data structure representing a distillation column in a chemical plant
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DistillationColumn {
    pub stages: u32,
    pub feed_positions: Vec<u32>,
    pub distilate_to_feed_ratio: f32,
    pub reflux_ratio: f32,
    pub temperature: f32,
    pub pressure: f32,
    // ...
    pub equations: Vec<String>,
    pub temperature_profile: Vec<f32>,
    pub pressure_profile: Vec<f32>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DistillationColumnBuilder {
    wip: DistillationColumn,
}

impl DistillationColumnBuilder {
    pub fn new() -> Self {
        DistillationColumnBuilder::default()
    }

    pub fn adapt(data: DistillationColumn) -> Self {
        DistillationColumnBuilder { wip: data }
    }

    pub fn set_stages(&mut self, stages: u32) -> &mut Self {
        self.wip.stages = stages;
        self
    }

    pub fn set_d2f_ratio(&mut self, d2f_ratio: f32) -> &mut Self {
        self.wip.distilate_to_feed_ratio = d2f_ratio;
        self
    }

    pub fn set_reflux_ratio(&mut self, reflux_ratio: f32) -> &mut Self {
        self.wip.reflux_ratio = reflux_ratio;
        self
    }

    pub fn set_temperature(&mut self, temp: f32) -> &mut Self {
        self.wip.temperature = temp;
        self
    }

    pub fn set_pressure(&mut self, pres: f32) -> &mut Self {
        self.wip.pressure = pres;
        self
    }

    pub fn add_feed_position(&mut self, feed_pos: u32) -> &mut Self {
        self.wip.feed_positions.push(feed_pos);
        self
    }

    pub fn clear_feed_positions(&mut self) -> &mut Self {
        self.wip.feed_positions.clear();
        self
    }

    fn check_err(&self) -> Result<(), String> {
        let mut errs = vec![];
        let d2f = self.wip.distilate_to_feed_ratio;
        if d2f < f32::EPSILON || d2f >= 1.0 {
            errs.push("Distilate to feed ratio must be between 0 and 1".to_string());
        }
        if self.wip.stages <= 2 {
            errs.push(format!("Stages={} must be greater than 2", self.wip.stages));
        }
        // ...

        if self.wip.reflux_ratio < f32::EPSILON {
            errs.push("Reflux ratio must be greater than 0".to_string());
        }
        if self.wip.feed_positions.is_empty() {
            errs.push("Needs at last one feed position".to_string());
        } else {
            for pos in self.wip.feed_positions.iter().map(|x| *x) {
                if pos < 2 || pos >= self.wip.stages {
                    errs.push(format!(
                        "Feed position {} not allowed range is 2-{}",
                        pos,
                        self.wip.stages - 1
                    ));
                }
            }
        }

        //...
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs.join("\n"))
        }
    }

    pub fn build(self) -> Result<DistillationColumn, String> {
        self.check_err().map(|_| self.wip)
    }

    pub fn build_and_reuse(&self) -> Result<DistillationColumn, String> {
        self.check_err().map(|_| self.wip.clone())
    }
}
