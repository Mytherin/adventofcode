const TARGET_X_MIN : i64 = 60;
const TARGET_X_MAX : i64 = 94;
const TARGET_Y_MIN : i64 = -171;
const TARGET_Y_MAX : i64 = -136;

// const TARGET_X_MIN : i64 = 20;
// const TARGET_X_MAX : i64 = 30;
// const TARGET_Y_MIN : i64 = -10;
// const TARGET_Y_MAX : i64 = -5;

fn check_in_target(x : i64, y : i64) -> bool {
    return x >= TARGET_X_MIN && x <= TARGET_X_MAX &&
           y >= TARGET_Y_MIN && y <= TARGET_Y_MAX;
}

fn will_never_hit_target(x : i64, y : i64, x_velocity : i64, y_velocity : i64) -> bool {
    if x > TARGET_X_MAX && x_velocity >= 0 {
        return true;
    }
    if y < TARGET_Y_MIN && y_velocity < 0 {
        return true;
    }
    if x < TARGET_X_MIN && x_velocity <= 0 {
        return true;
    }
    if y > 10000000 {
        return true;
    }
    return false;
}

fn run_simulation(initial_x_velocity : i64, initial_y_velocity : i64) -> i64 {
    let mut x : i64 = 0;
    let mut y : i64 = 0;
    let mut x_velocity : i64 = initial_x_velocity;
    let mut y_velocity : i64 = initial_y_velocity;

    let mut max_y = 0;
    let mut hit_target = false;
    // println!("start simulation");
    loop {
        x += x_velocity;
        y += y_velocity;
        if y > max_y {
            max_y = y;
        }
        if x_velocity > 0 {
            x_velocity -= 1;
        } else if x_velocity < 0 {
            x_velocity += 1;
        }
        y_velocity -= 1;
        if check_in_target(x, y) {
            return max_y;
        }
        if will_never_hit_target(x, y, x_velocity, y_velocity) {
            break;
        }
    }
    if hit_target {
        return max_y;
    }
    return 0;
}

fn main() {
    let mut max_y_found : i64 = 0;
    for x_velocity in 0..TARGET_X_MAX {
        for y_velocity in 0..TARGET_X_MAX+10000000 {
            let max_y = run_simulation(x_velocity, y_velocity);
            if max_y > max_y_found {
                max_y_found = max_y;
            }
        }
    }
    println!("{}", max_y_found);
}
