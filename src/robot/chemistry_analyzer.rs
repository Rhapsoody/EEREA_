use crate::robot::Robot;

impl ChemistryAnalyzer for Robot {
	fn analyze(&self) {
			println!("ChemistryAnalyzer {} is analyzing", self.id);
	}
}
 