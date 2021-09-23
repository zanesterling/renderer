use crate::data::*;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scene {
    pub commands: Vec<Command>,
    vars: HashMap<String, Vec<Animation>>
}

#[derive(Debug)]
struct Animation {
    from: f32,
    to: f32,
    t1: f32,
    t2: f32,
}

impl Animation {
    fn overlaps(&self, other: &Animation) -> bool {
        (self.t1 <= other.t1 && other.t1 <= self.t2) ||
        (self.t1 <= other.t2 && other.t2 <= self.t2) ||
        (other.t1 <= self.t1 && self.t1 <= other.t2) ||
        (other.t1 <= self.t2 && self.t2 <= other.t2)
    }
}

impl Scene {
    pub fn eval_at(&self, time: f32, val: &Val) -> Result<f32, String> {
        let var = match val {
            Val::Raw(x) => return Ok(*x),
            Val::Var(s) => s,
        };

        let mut last_time = -1.0;
        let mut last_val = None;
        let anims = self.vars.get(var)
            .ok_or(format!("var \"{}\" not defined", var))?;
        for anim in anims {
            match *anim {
                Animation { from, to, t1, t2 } =>
                    if t1 <= time && time <= t2 {
                        let p = (time - t1) / (t2 - t1);
                        return Ok(lerp(from, to, p));
                    } else if time > t2 && t2 > last_time {
                        last_time = t2;
                        last_val = Some(to);
                    }
            }
        }

        match last_val {
            Some(x) => Ok(x),
            None => Err(
                format!("var \"{}\" has no matching animations at time {:?}", var, time)),
        }
    }
}

fn lerp(y1: f32, y2: f32, t: f32) -> f32 {
    y1 * (1.0-t) + y2 * t
}


#[derive(Debug, Clone)]
pub enum Val {
    Raw(f32),
    Var(String),
}

#[derive(Debug)]
pub enum Command {
    Point { x: Val, y: Val, z: Val, rad: Val },
    Line(Point3, Point3),
    Triangle(Point3, Point3, Point3),

    Identity,
    Translate(f32, f32, f32),
    Scale(f32, f32, f32),
    Rotate { theta: Val, vx: Val, vy: Val, vz: Val },

    Color(Color),
}

pub fn load_scene(path: &str) -> Result<Scene, String> {
    let mut lines = read_lines(path)
        .map_err(|_| { format!("file \"{}\" does not exist", path) })?;

    let mut commands: Vec<Command> = vec![];
    let mut vars: HashMap<String, Vec<Animation>> = HashMap::new();

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
        match cmd {
            "#" => continue 'foo,

            "point"    => commands.push(parse_cmd_point(&mut lines)?),
            "line"     => commands.push(parse_cmd_line(&mut lines)?),
            "triangle" => commands.push(parse_cmd_triangle(&mut lines)?),

            "identity"  => commands.push(Command::Identity),
            "translate" => commands.push(parse_cmd_translate(&mut lines)?),
            "scale"     => commands.push(parse_cmd_scale(&mut lines)?),
            "rotate"    => commands.push(parse_cmd_rotate(&mut lines)?),

            "color"     => commands.push(parse_cmd_color(&mut lines)?),
            "animate"   => {
                let (var, animation) = parse_cmd_animate(&mut lines)?;
                if !vars.contains_key(&var) {
                    vars.insert(var.clone(), vec![]);
                }
                let anims = vars.get_mut(&var).unwrap();
                for i in 0..anims.len() {
                    let other = &anims[i];
                    if animation.t2 < other.t1 {
                        anims.insert(i, animation);
                        continue 'foo;
                    }
                    if animation.overlaps(&other) {
                        return Err(
                            format!("animation for var \"{}\" overlaps with another", var));
                    }
                }
                anims.push(animation);
            }

            _ => return Err(format!("line \"{}\" does not have a command", line)),
        };
    }

    Ok(Scene {
        commands: commands,
        vars: vars,
    })
}

fn ran_out_of_lines(cmd_name: &str) -> String {
    format!("ran out of lines while parsing command \"{}\"", cmd_name)
}

fn parse_cmd_point(
    lines: &mut io::Lines<io::BufReader<File>>
) -> Result<Command, String> {
    let line = lines.next().ok_or(ran_out_of_lines("point"))?;
    let line = line.map_err(|e| e.to_string())?;
    let fs = parse_n_vals(4, line)?;
    Ok(Command::Point{
        x: fs[0].clone(),
        y: fs[1].clone(),
        z: fs[2].clone(),
        rad: fs[3].clone(),
    })
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

fn parse_cmd_translate(
    lines: &mut io::Lines<io::BufReader<File>>
) -> Result<Command, String> {
    let l = lines.next().ok_or(ran_out_of_lines("line"))?;
    let l = l.map_err(|e| e.to_string())?;
    let xs = parse_n_floats(3, l)?;
    Ok(Command::Translate(xs[0], xs[1], xs[2]))
}

fn parse_cmd_scale(
    lines: &mut io::Lines<io::BufReader<File>>
) -> Result<Command, String> {
    let l = lines.next().ok_or(ran_out_of_lines("line"))?;
    let l = l.map_err(|e| e.to_string())?;
    let xs = parse_n_floats(3, l)?;
    Ok(Command::Scale(xs[0], xs[1], xs[2]))
}

fn parse_cmd_rotate(lines: &mut io::Lines<io::BufReader<File>>) -> Result<Command, String> {
    let xs = parse_n_vals(4, next_line(lines)?)?;
    Ok(Command::Rotate{
        theta: xs[0].clone(),
        vx: xs[1].clone(),
        vy: xs[2].clone(),
        vz: xs[3].clone()
    })
}

fn parse_cmd_color(
    lines: &mut io::Lines<io::BufReader<File>>
) -> Result<Command, String> {
    let xs = parse_n_u8s(3, next_line(lines)?)?;
    Ok(Command::Color(Color { r: xs[0], g: xs[1], b: xs[2] }))
}

fn parse_cmd_animate(
    lines: &mut io::Lines<io::BufReader<File>>
) -> Result<(String, Animation), String> {
    let l = next_line(lines)?;
    let (var, rest) = l.split_once(" ")
        .ok_or("line does not have a first thing")?;
    let xs = parse_n_floats(4, rest.to_string())?;
    Ok((
        var.to_string(),
        Animation { from: xs[0], to: xs[1], t1: xs[2], t2: xs[3]}))
}

fn next_line(
    lines: &mut io::Lines<io::BufReader<File>>
) -> Result<String, String> {
    let l = lines.next().ok_or(ran_out_of_lines("line"))?;
    l.map_err(|e| e.to_string())
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

fn parse_n_vals(
    n: usize,
    line: String,
) -> Result<Vec<Val>, String> {
    let xs: Vec<Val> = line.split(" ")
        .map(|s| s.parse()
            .map_or(Val::Var(s.to_string()), |f| Val::Raw(f)))
        .collect::<Vec<Val>>();
    if xs.len() == n {
        return Ok(xs);
    }
    Err(format!("expected {} values, found {}", n, xs.len()))
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}