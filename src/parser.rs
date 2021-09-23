use crate::data::*;

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::path::Path;

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
        (self.t1 < other.t1 && other.t1 < self.t2) ||
        (self.t1 < other.t2 && other.t2 < self.t2) ||
        (other.t1 < self.t1 && self.t1 < other.t2) ||
        (other.t1 < self.t2 && self.t2 < other.t2)
    }
}

impl Scene {
    fn eval_at(&self, time: f32, val: &Val) -> Result<f32, String> {
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

pub trait Eval {
    type Out;
    fn eval_at(&self, time: f32, scene: &Scene) -> Result<Self::Out, String>;
}

impl Eval for Val {
    type Out = f32;
    fn eval_at(&self, t: f32, scene: &Scene) -> Result<Self::Out, String> {
        scene.eval_at(t, self)
    }
}

impl Eval for ValPoint3 {
    type Out = Point3;
    fn eval_at(&self, t: f32, scene: &Scene) ->  Result<Self::Out, String> {
        Ok(Point3 {
            x: self.x.eval_at(t, scene)?,
            y: self.y.eval_at(t, scene)?,
            z: self.z.eval_at(t, scene)?,
        })
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
pub struct ValPoint3 {
    x: Val,
    y: Val,
    z: Val
}

// ====================================================================== //
// ============================== COMMANDS ============================== //
// ====================================================================== //

#[derive(Debug)]
pub enum Command {
    Point { p: ValPoint3, rad: Val },
    Line(ValPoint3, ValPoint3),
    Triangle(ValPoint3, ValPoint3, ValPoint3),
    Mesh { points: Vec<Point3>, triangles: Vec<usize> },

    Identity,
    Translate(Val, Val, Val),
    Scale(Val, Val, Val),
    Rotate { theta: Val, v: ValPoint3 },

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
            Some(Err(e)) => return Err(format!("bad line parse: {}", e)),
            Some(Ok(l)) => l
        };

        if line == "" { continue }
        let (cmd, rest) = line.trim().split_once(" ").unwrap_or((&line, ""));
        match &*cmd.to_lowercase() {
            "#" => continue 'foo,

            "point"    => commands.push(parse_cmd_point(rest)?),
            "line"     => commands.push(parse_cmd_line(rest)?),
            "triangle" => commands.push(parse_cmd_triangle(rest)?),
            "mesh"     => commands.push(parse_cmd_mesh(rest)?),

            "identity"  => commands.push(Command::Identity),
            "translate" => commands.push(parse_cmd_translate(rest)?),
            "scale"     => commands.push(parse_cmd_scale(rest)?),
            "rotate"    => commands.push(parse_cmd_rotate(rest)?),

            "color"     => commands.push(parse_cmd_color(rest)?),
            "animate"   => {
                let (var, animation) = parse_cmd_animate(rest)?;
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

fn parse_cmd_point(rest: &str) -> Result<Command, String> {
    let fs = parse_n_vals(4, rest)?;
    Ok(Command::Point{
        p: ValPoint3{ x: fs[0].clone(), y: fs[1].clone(), z: fs[2].clone() },
        rad: fs[3].clone(),
    })
}

fn parse_cmd_line(rest: &str) -> Result<Command, String> {
    let xs = parse_n_vals(6, rest)?;
    Ok(Command::Line(
        ValPoint3 { x: xs[0].clone(), y: xs[1].clone(), z: xs[2].clone() },
        ValPoint3 { x: xs[3].clone(), y: xs[4].clone(), z: xs[5].clone() }
    ))
}

fn parse_cmd_triangle(rest: &str) -> Result<Command, String> {
    let fs = parse_n_vals(9, rest)?;
    Ok(Command::Triangle(
        ValPoint3 { x: fs[0].clone(), y: fs[1].clone(), z: fs[2].clone() },
        ValPoint3 { x: fs[3].clone(), y: fs[4].clone(), z: fs[5].clone() },
        ValPoint3 { x: fs[6].clone(), y: fs[7].clone(), z: fs[8].clone() }
    ))
}

fn parse_cmd_mesh(rest: &str) -> Result<Command, String> {
    let path = rest.trim_matches('"');
    if rest.len() - path.len() != 2 { return Err("expected \" enclosed filepath".to_string()); }
    let obj_lines = &mut read_lines(path)
        .map_err(|_| { format!("file \"{}\" does not exist", path) })?;

    let mut points = vec![];
    let mut triangles = vec![];

    let line = next(obj_lines)?;
    if line != "points" { return Err("expected first line to be \"points\"".to_string()); }
    loop {
        let line = next(obj_lines)?;
        if line == "triangles" { break; }
        let fs = parse_n_floats(3, line.trim())?;
        points.push(Point3 { x: fs[0], y: fs[1], z: fs[2] });
    }
    loop {
        match next(obj_lines) {
            Ok(line) => {
                let xs = parse_n_u8s(3, line.trim())?;
                triangles.push(xs[0] as usize);
                triangles.push(xs[1] as usize);
                triangles.push(xs[2] as usize);
            },
            _ => break
        }
    }
    Ok(Command::Mesh { points: points, triangles: triangles })
}

fn parse_cmd_translate(rest: &str) -> Result<Command, String> {
    let xs = parse_n_vals(3, rest)?;
    Ok(Command::Translate(xs[0].clone(), xs[1].clone(), xs[2].clone()))
}

fn parse_cmd_scale(rest: &str) -> Result<Command, String> {
    let xs = parse_n_vals(3, rest)?;
    Ok(Command::Scale(xs[0].clone(), xs[1].clone(), xs[2].clone()))
}

fn parse_cmd_rotate(rest: &str) -> Result<Command, String> {
    let xs = parse_n_vals(4, rest)?;
    Ok(Command::Rotate{
        theta: xs[0].clone(),
        v: ValPoint3 { x: xs[1].clone(), y: xs[2].clone(), z: xs[3].clone() }
    })
}

fn parse_cmd_color(rest: &str) -> Result<Command, String> {
    let xs = parse_n_u8s(3, rest)?;
    Ok(Command::Color(Color { r: xs[0], g: xs[1], b: xs[2] }))
}

fn parse_cmd_animate(rest: &str) -> Result<(String, Animation), String> {
    let (var, rest) = rest.split_once(" ")
        .ok_or("line does not have a first thing")?;
    let xs = parse_n_floats(4, rest)?;
    Ok((
        var.to_string(),
        Animation { from: xs[0], to: xs[1], t1: xs[2], t2: xs[3]}))
}

fn parse_n_u8s(
    n: usize,
    line: &str,
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
    line: &str,
) -> Result<Vec<f32>, String> {
    let xs: Vec<f32> = line.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().map_err(|e| format!("parsing \"{}\": {}", s, e)))
        .collect::<Result<Vec<f32>, _>>()
        .map_err(|e| e.to_string())?;
    if xs.len() == n {
        return Ok(xs);
    }
    Err(format!("expected {} floats, found {}", n, xs.len()))
}

fn parse_n_vals(
    n: usize,
    line: &str,
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

fn next(lines: &mut io::Lines<io::BufReader<File>>) -> Result<String, String> {
    lines.next().ok_or(ran_out_of_lines("line"))?
        .map_err(|e| e.to_string())
}