fn question1() {
    let mut seconds = 1_666_800u32;
    let week_secs = 7 * 24 * 3600;
    let day_secs = 24 * 3600;
    let hour_secs = 3600;

    let weeks = seconds / week_secs;
    seconds %= week_secs;
    let days = seconds / day_secs;
    seconds %= day_secs;
    let hours = seconds / hour_secs;

    println!("Philipp has waited {weeks} weeks, {days} days, and {hours} hours")
}

#[allow(dead_code)]
fn question1_float() {
    let mut seconds: f32 = 1_666_800.0;
    let weeks = (seconds / (7. * 24. * 3600.)).trunc();
    seconds -= weeks * 7. * 24. * 3600.;
    let days = (seconds / (24. * 3600.)).trunc();
    seconds -= days * 24. * 3600.;
    let hours = (seconds / 3600.).trunc();
    println!("Philipp waited {weeks} weeks, {days} days, and {hours} hours")
}

fn question2() {
    let x: u32 = 92;
    let y: u32 = 4231;
    let z: u32 = 4232;
    println!("Int: {}", x.pow(2) + y.pow(2) - z.pow(2));

    let x: f32 = 92.;
    let y: f32 = 4231.;
    let z: f32 = 4232.;
    println!("Float: {:.2}", x.powi(2) + y.powi(2) - z.powi(2));
}

fn main() {
    question1();
    question2();
}
