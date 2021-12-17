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
    return false;
}

fn run_simulation(initial_x_velocity : i64, initial_y_velocity : i64) -> bool {
    let mut x : i64 = 0;
    let mut y : i64 = 0;
    let mut x_velocity : i64 = initial_x_velocity;
    let mut y_velocity : i64 = initial_y_velocity;

    loop {
        x += x_velocity;
        y += y_velocity;
        if x_velocity > 0 {
            x_velocity -= 1;
        } else if x_velocity < 0 {
            x_velocity += 1;
        }
        y_velocity -= 1;
        if check_in_target(x, y) {
            return true;
        }
        if will_never_hit_target(x, y, x_velocity, y_velocity) {
            return false;
        }
    }
}

fn main() {
    // the max we can get is -171
    // we surpass the target after TARGET_X_MAX / x_velocity steps
    // println!("{}", run_simulation(6, 9));
    let mut hit_count : i64 = 0;
    for x_velocity in 0..TARGET_X_MAX+1 {
        for y_velocity in TARGET_Y_MIN..500 {
            if run_simulation(x_velocity, y_velocity) {
                hit_count += 1;
            }
        }
    }
    println!("{}", hit_count);
}
