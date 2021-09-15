

pub type Point = (f64, f64);
pub fn read_data(path: &str) -> Vec<Point> {
    use std::fs::read;
    let content = read(path).expect("Path not found");
    let content = String::from_utf8(content).expect("not valid utf8");

    content.lines().skip(1).map(|line| {
        let mut points = line.split(",").map(|s| s.parse::<f64>().expect("invalid file content, must be number"));

            (
            points.next().unwrap(),
            points.next().unwrap(),
            )
    }).collect()
}
