use std::cmp;
use std::io;
use std::ops;

use itertools::Itertools;

type Coords = (i32, i32);
type Range = ops::RangeInclusive<i32>;

#[derive(Debug)]
pub enum Status {
    Hit(Coords),
    TooClose(i32),
    TooFar(i32),
    TooLow(i32),
    Undershoot(Coords),
    Overshoot(Coords),
    Unknown,
}

#[derive(Debug)]
pub struct Target {
    x: Range,
    y: Range,
}

/// Compute Σ(n..=m).
pub fn sum_series(n: i32, m: i32) -> i32 {
    (m * (m+1) - (n-1) * n) / 2
}

/// Find n such that Σ(1..=n) == s.
/// Returns a float (since s is not necessarily the exact sum of a series), .floor() or .ceil() it
/// depending on what you need.
fn unsum_series(s: i32) -> f32 {
    // n * (n+1) / 2 = s   <=>   n² + n - 2s = 0
    let d = 1 + 8*s;
    (-1f32 + (d as f32).sqrt()) / 2f32
}

/// Find n such that Σ(n..=n+c) == s.
/// Returns a float (since s is not necessarily the exact sum of a series), .floor() or .ceil() it
/// depending on what you need.
fn unsum_partial_series(s: i32, c: i32) -> f32 {
    // Σ(n..=n+c) == n*c + Σ(1..=c)
    let steps = sum_series(1, c);
    (s - steps) as f32 / c as f32
}

impl Target {
    pub fn contains(&self, probe: &Probe) -> bool {
        self.x.contains(&probe.pos.0) && self.y.contains(&probe.pos.1)
    }

    /// Returns the range of X velocities that have a chance of hitting the target.
    pub fn x_velocities(&self) -> Range {
        // Lower bound: N such that x.start <= Σ(1..=N). X velocities below that will fall short of
        //              the target.
        let lower = unsum_series(*self.x.start()).ceil() as i32;
        // Upper bound: velocity such that the probe reaches x.end in 1 step.
        let upper = *self.x.end();
        lower..=upper
    }

    /// For a given X velocity, returns the range of Y velocities that have a chance of hitting the
    /// target (there may be none).
    pub fn y_velocities(&self, x_velocity: i32) -> Option<Range> {
        // NB: This all assumes the target area is below 0 in Y coordinates.
        let sum_x = sum_series(1, x_velocity);
        if sum_x < *self.x.start() {
            // Can't reach the target, the X velocity is too low.
            None
        } else if sum_x <= *self.x.end() {
            // We will reach 0 X velocity within the target, possible Y velocities range from
            // direct shot (the lowest of which will nearly undershoot) to high lob (the highest of
            // which will nearly fall through).
            // Direct shot: we will undershoot if we're below the target once we reach its X range.
            // - Compute the number of steps to reach the target's X range.
            let steps_within = unsum_series(sum_x - *self.x.start()).floor() as i32;
            let steps_to_x = x_velocity - steps_within;
            // - Now we need the highest downwards initial Y velocity that still reaches the target
            //   in this many steps.
            let lower = -unsum_partial_series(0 - *self.y.start(), steps_to_x).floor() as i32;
            // High lob: rising and falling are symmetric, so during the fall we will cross Y
            //           coordinate 0 with our initial Y velocity + 1, only negated. We're sure to
            //           fall through the target if this Y velocity is greater than the target's
            //           lowest Y coordinate.
            let upper = self.y.start().abs() - 1;
            Some(lower..=upper)
        } else {
            // We will overshoot if we lob too high, so we're restricted to mostly direct shots
            // (from nearly undershooting to nearly overshooting).
            // The lower bound is the same as above (duplicate code I know).
            let steps_within = unsum_series(sum_x - *self.x.start()).floor() as i32;
            let steps_to_x_start = x_velocity - steps_within;
            let lower = -unsum_partial_series(0 - *self.y.start(), steps_to_x_start).floor() as i32;
            // The upper bound works similarly:
            // - Compute the number of steps to reach the end of the target's X range.
            let steps_without = unsum_series(sum_x - *self.x.end()).ceil() as i32;
            let steps_to_x_end = x_velocity - steps_without;
            // - Now we need the highest upwards initial Y velocity that still reaches the target
            //   in this many steps.
            //   - If we lob somewhat, it will take 2*Vy+1 steps to cross Y coordinate 0 again.
            //     This means we can only lob for a limited number of steps (hence a limited
            //     initial Y velocity), and we'll need sufficient velocity afterwards to reach the
            //     target.
            let velocity_for_max_lob = (steps_to_x_end / 2) - 1;
            let required_velocity_after_max_lob = 0 - *self.y.start();
            let upper = if velocity_for_max_lob < required_velocity_after_max_lob - 1 {
                // So max-lobing doesn't reach deep enough. That's ok, it's an overshoot, and is
                // hence still a valid upper bound I guess? (I'm tired after all this thinking.)
                velocity_for_max_lob
            } else {
                // We can max-lob (although our downwards velocity after the lob may still be to
                // high to hit the target, but I'm tired and won't take this into account, I only
                // need bounds after all).
                velocity_for_max_lob
            };
            Some(lower..=upper)
        }
    }
}

pub struct Probe {
    velocity: Coords,
    pos: Coords,
    highest: i32,
}

impl Probe {
    pub fn new(velocity: Coords) -> Self {
        Probe {
            velocity,
            pos: (0,0),
            highest: 0,
        }
    }

    pub fn velocity(&self) -> Coords { self.velocity }
    pub fn highest(&self) -> i32 { self.highest }

    pub fn step(&mut self) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;
        self.velocity.0 -= if self.velocity.0 > 0 { 1 } else if self.velocity.0 < 0 { -1 } else { 0 };
        self.velocity.1 -= 1;

        self.highest = cmp::max(self.pos.1, self.highest);
    }

    pub fn check_target(&self, target: &Target) -> Status {
        if target.contains(&self) {
            Status::Hit(self.pos)
        } else if self.max_x() < *target.x.start() {
            Status::TooClose(self.max_x())
        } else if *target.x.end() < self.pos.0 {
            if *target.y.end() < self.pos.1 {
                Status::Overshoot(self.pos)
            } else {
                Status::TooFar(self.pos.0)
            }
        } else if self.pos.1 < *target.y.start() {
            if self.pos.0 < *target.x.start() {
                Status::Undershoot(self.pos)
            } else {
                Status::TooLow(self.pos.1)
            }
        } else {
            Status::Unknown
        }
    }


    pub fn simulate(&mut self, target: &Target) -> Status {
        let mut status = self.check_target(target);
        while matches!(status, Status::Unknown) {
            self.step();
            status = self.check_target(target);
        }
        status
    }

    pub fn max_x(&self) -> i32 {
        self.pos.0 + sum_series(1, self.velocity.0)
    }

    pub fn max_y(&self) -> i32 {
        let dy = self.velocity.1;
        let y = if dy > 0 { sum_series(1, dy) } else { 0 };
        self.pos.1 + y
    }
}

pub fn parse_stdin() -> Target {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let (x, y) = line.trim().split(", ").map(|s| {
        let (a,b) = s.split('=').nth(1).unwrap().split("..").map(
            |n| n.parse::<i32>().unwrap()
        ).collect_tuple().unwrap();
        a..=b
    }).collect_tuple().unwrap();
    Target { x, y, }
}
