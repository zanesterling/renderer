use crate::data::*;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct Scene {
    pub commands: Vec<Command>,
}

#[derive(Debug)]
pub enum Command {
    Point(Point3, f32),
    Line(Point3, Point3),
    Triangle(Point3, Point3, Point3),

    // Translate(f32, f32, f32),
    Scale(f32, f32, f32),
    // Rotate(Point3, f32),

    Color(Color),
}

pub fn load_scene(path: &str) -> Result<Scene, String> {
    let mut lines = read_lines(path)
        .map_err(|_| { format!("file \"{}\" does not exist", path) })?;

    let mut commands: Vec<Command> = vec![];

    'foo: loop {
        let line = match lines.next() {
            None => break 'foo,
            Some(Err(e)) => format!("bad line parse: {}", e),
            Some(Ok(l)) => l
        };

        if line == "" { continue }
        let cmd = line.trim()
            .split(" ")
            .next()
            .ok_or("line \"{}\" does not have a command")?;
        let cmd = match cmd {
            "#" => continue 'foo,

            "point"    => parse_cmd_point(&mut lines),
            "line"     => parse_cmd_line(&mut lines),
            "triangle" => parse_cmd_triangle(&mut lines),

            "translate" => parse_cmd_translate(&mut lines),
            "scale"     => parse_cmd_scale(&mut lines),
            "rotate"    => parse_cmd_rotate(&mut lines),

            "color"     => parse_cmd_color(&mut lines),

            _ => Err(format!("line \"{}\" does not have a command", line)),
        };
        commands.push(cmd?);
    }

    Ok(Scene { commands: commands })
}

fn ran_out_of_lines(cmd_name: &str) -> String {
    format!("ran out of lines while parsing command \"{}\"", cmd_name)
}

fn parse_cmd_point(
    lines: &mut io::Lines<io::BufReader<File>>
) -> Result<Command, String> {
    let line = lines.next().ok_or(ran_out_of_lines("point"))?;
    let line = line.map_err(|e| e.to_string())?;
    let fs = parse_n_floats(4, line)?;
    Ok(Command::Point(
        Point3 { x: fs[0], y: fs[1], z: fs[2] },
        fs[3]
    ))
}

fn parse_cmd_line(
    lines: &mut io::Lines<io::BufReader<File>>
) -> Result<Command, String> {
    let l = lines.next().ok_or(ran_out_of_lines("line"))?;
    let l = l.map_err(|e| e.to_string())?;
    let fs = parse_n_floats(6, l)?;
    Ok(Command::Line(
        Point3 { x: fs[0], y: fs[1], z: fs[2] },
        Point3 { x: fs[3], y: fs[4], z: fs[5] }
    ))
}

fn parse_cmd_triangle(
    lines: &mut io::Lines<io::BufReader<File>>
) -> Result<Command, String> {
    let l = lines.next().ok_or(ran_out_of_lines("line"))?;
    let l = l.map_err(|e| e.to_string())?;
    let fs = parse_n_floats(9, l)?;
    Ok(Command::Triangle(
        Point3 { x: fs[0], y: fs[1], z: fs[2] },
        Point3 { x: fs[3], y: fs[4], z: fs[5] },
        Point3 { x: fs[6], y: fs[7], z: fs[8] }
    ))
}

fn parse_cmd_translate(_lines: &mut io::Lines<io::BufReader<File>>) -> Result<Command, String> { unimplemented!() }

fn parse_cmd_scale(
    lines: &mut io::Lines<io::BufReader<File>>
) -> Result<Command, String> {
    let l = lines.next().ok_or(ran_out_of_lines("line"))?;
    let l = l.map_err(|e| e.to_string())?;
    let xs = parse_n_floats(3, l)?;
    Ok(Command::Scale(xs[0], xs[1], xs[2]))
}

fn parse_cmd_rotate(_lines: &mut io::Lines<io::BufReader<File>>) -> Result<Command, String> { unimplemented!() }

fn parse_cmd_color(
    lines: &mut io::Lines<io::BufReader<File>>
) -> Result<Command, String> {
    let l = lines.next().ok_or(ran_out_of_lines("line"))?;
    let l = l.map_err(|e| e.to_string())?;
    let xs = parse_n_u8s(3, l)?;
    Ok(Command::Color(Color { r: xs[0], g: xs[1], b: xs[2] }))
}

fn parse_n_u8s(
    n: usize,
    line: String,
) -> Result<Vec<u8>, String> {
    let xs: Vec<u8> = line.split(" ")
        .map(|s| s.parse())
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|e| e.to_string())?;
    if xs.len() == n {
        return Ok(xs);
    }
    Err(format!("expected {} u8s, found {}", n, xs.len()))
}

fn parse_n_floats(
    n: usize,
    line: String,
) -> Result<Vec<f32>, String> {
    let xs: Vec<f32> = line.split(" ")
        .map(|s| s.parse())
        .collect::<Result<Vec<f32>, _>>()
        .map_err(|e| e.to_string())?;
    if xs.len() == n {
        return Ok(xs);
    }
    Err(format!("expected {} floats, found {}", n, xs.len()))
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}