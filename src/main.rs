use eerea::robot::{base::RobotActions, Robot, RobotBehavior, RobotSpecialization};


fn main() {
    let robot = Robot::new(1, RobotBehavior::Explore, RobotSpecialization::Driller);

    robot.return_to_station();
}