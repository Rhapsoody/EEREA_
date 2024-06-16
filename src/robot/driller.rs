use crate::robot::Robot;

impl Driller for Robot {
	fn drill(&self) {
			println!("Driller {} is drilling", self.id);
	}
}
 