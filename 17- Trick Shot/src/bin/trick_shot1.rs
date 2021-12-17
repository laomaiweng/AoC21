use trick_shot::{parse_stdin, sum_series};

fn main() {
    let target = parse_stdin();
    let vx = *target.x_velocities().start();
    let vy = *target.y_velocities(vx).unwrap().end();
    println!("{:?}", target);
    println!("Minimum X velocity: {} (max X: {})", vx, sum_series(1, vx));
    println!("Maximum Y velocity: {} (max Y: {})", vy, sum_series(1, vy));

    // for x in min_x_velocity..=target.max_x_velocity() {
    //     for y_velocity in 0..200 {
    //         let mut probe = Probe::new((min_x_velocity, y_velocity));

    //         let status = probe.simulate(&target);
    //         println!("Simulating probe with initial velocity ({},{}): {:?}", min_x_velocity, y_velocity, status);
    //         if matches!(status, Status::Hit(_)) {
    //             println!(" Highest point: {}", probe.highest());
    //             println!(" Terminal velocity: {:?}", probe.velocity());
    //         }
    //     }
    // }
}
