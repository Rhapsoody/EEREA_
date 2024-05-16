use crate::robot::Robot;

impl Physicist for Robot {
	fn calculate(&self) {
			println!("Physicist {} is calculating", self.id);
	}
}