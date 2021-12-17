use std::env;

// x, y
type Point = (i32,i32);
// min x, max x, min y, max y
type Area = ((i32, i32), (i32, i32));

fn main() {
    let args: Vec<String> = env::args().collect();
    let (version, filename) = parse_config(&args);
    println!("In file {}", filename);
    let input = read_input(version);
    if version != "2" {
        return v1(&input);
    }
    //return v2(&input);
}

fn v1(target_area: &Area) {
    println!("target area {:?}", target_area);
    let mut total_max_y = -100;
    let mut hits = 0;
    for vx in -500..500 {
        for vy in -500..500 {
            let pos = (0,0);
            let velocity = (vx,vy);
            let (hit, max_y) = do_steps(target_area, pos, velocity);
            if hit {
                hits += 1;
                println!("hit, velocity {:?} max y {:?}", velocity, max_y);
                if max_y > total_max_y {
                    total_max_y = max_y;
                }
            }
        }
    }
    println!("total max y {}, hits {}", total_max_y, hits);
}

fn do_steps(target_area: &Area, initial_pos: Point, initial_velocity: Point)-> (bool, i32) {
    let mut hit = false;
    let mut pos = [initial_pos.0, initial_pos.1];
    let mut velocity = [initial_velocity.0, initial_velocity.1];
    let mut max_y = -100;
    //println!("trying velocity {:?}", velocity);
    for _ in 1..5000 {
        //println!("step {:?} {:?}", pos, velocity);
        pos[0] += velocity[0];
        pos[1] += velocity[1];
        if velocity[0] > 0 {
            velocity[0] -= 1;
        } else if velocity[0] < 0 {
            velocity[0] += 1;
        }
        velocity[1] -= 1;

        // update max y
        if pos[1] > max_y {
            max_y = pos[1];
        }

        // Hit target?
        if pos[0] >= target_area.0.0 && pos[0] <= target_area.0.1 &&
           pos[1] >= target_area.1.0 && pos[1] <= target_area.1.1 {
            hit = true;
        } else if pos[1] < target_area.1.0 /*|| pos[0] > target_area.0.1*/ {
            // passed target?
            //println!("passed {:?}", pos);
            break
        }
    }
    if hit {
        return (hit, max_y);
    }
    (hit, -100)
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let version = &args[1];
    let filename = &args[2];

    (version, filename)
}

fn read_input(version: &str) -> Area {
    if version == "test" {
        return ((20,30),(-10,-5))
    } else {
        return ((25,67),(-260,-200))
    }
}