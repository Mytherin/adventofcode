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
    // X bounds are positive -> don't need to consider negative X velocities
    // we don't need to consider numbers bigger than TARGET_X_MAX
    // -> we will always overshoot in that case
    // so our range is 0..TARGET_X_MAX + 1

    // for Y, we need to consider the numbers between the TARGET_Y_MIN
    // -> in this case we will be in the target y zone after one step
    // and -TARGET_Y_MIN
    // the reason this is our upper bound is that the y velocity is symmetric
    // for example, for y velocity 3, our trajectory is
    // 0 -> 3 -> 5 -> 6 -> 6 -> 5 -> 3 -> 0 -> -4
    // we arrive back at location 0 with a speed of -initial_velocity
    // this means that if our initial velocity is bigger than -TARGET_Y_MIN
    // we will always overshoot our target
    let mut hit_count : i64 = 0;
    for x_velocity in 0..TARGET_X_MAX+1 {
        for y_velocity in TARGET_Y_MIN..-TARGET_Y_MIN {
            if run_simulation(x_velocity, y_velocity) {
                hit_count += 1;
            }
        }
    }
    println!("{}", hit_count);
}
