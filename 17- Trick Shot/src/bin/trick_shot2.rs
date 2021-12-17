use trick_shot::{Probe, Status, parse_stdin};

fn main() {
    let target = parse_stdin();
    println!("{:?}", target);

    #[cfg(not(feature = "nuke"))]
    let x_velocities = target.x_velocities();
    #[cfg(feature = "nuke")]
    let x_velocities = 0..=1000;
    println!("X velocities: {:?}", x_velocities);

    let mut hits = 0;
    let mut count = 0;
    for vx in x_velocities {
        #[cfg(not(any(feature = "brute", feature = "nuke")))]
        let y_velocities = target.y_velocities(vx).unwrap();
        #[cfg(any(feature = "brute", feature = "nuke"))]
        let y_velocities = -1000..=1000;
        for vy in  y_velocities {
            let mut probe = Probe::new((vx, vy));
            count += 1;

            let status = probe.simulate(&target);
            // println!("Simulating probe with initial velocity ({},{}): {:?}", vx, vy, status);
            if matches!(status, Status::Hit(_)) {
                hits += 1;
            }
        }
    }

    println!("Hits: {}/{}", hits, count);
}
