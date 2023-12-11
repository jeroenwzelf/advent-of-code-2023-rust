use crate::days::Day;

pub struct Day06a;

impl Day for Day06a {
    fn run(input: &str) -> String {
        // get times and distances iterators
        let [ref mut times, ref mut distances, ..] = input.lines()
            .take(2)
            .map(|line| line.split_whitespace()
                .skip(1)
                .map(|numbers| numbers.parse::<usize>().unwrap()))
            .collect::<Vec<_>>()[..] else { panic!("Wrong input!") };

        times.map(|time| {
            let distance_record = distances.next().unwrap();
            // The solution can be written as a quadratic formula.
            // x = button_hold_time, where button_hold_time must get win the distance_threshold:
            //      distance_record <   x*(time-x)
            //      0               <   -x^2 + time * x - distance_record
            //      0               =   ax^2 + bx + c = 0, a = -1, b = time, c = -distance_record
            //
            // The solution to this quadratic formula can be described as the lower threshold of
            // x (button_hold_time) resulting in a better outcome than the distance_threshold.
            //      x = (-b -/+ sqrt(b^2 - 4ac)) / 2a
            //      x = (-time -/+ sqrt(time*time - 4(distance_record)) / -2
            //      x = (time -/+ sqrt(time*time - 4(distance_record)) / 2
            let x1 = (time - f64::sqrt((time * time - 4 * distance_record) as f64) as usize) / 2;
            // (time + sqrt(time*time - 4(distance_record)) / 2 == time - ((time - sqrt(time*time - 4(distance_record)) / 2)
            let x2 = time - x1;
            x2 - x1 + 1
        }).product::<usize>().to_string()
    }
}