use std::sync::Mutex;
use crate::utils::split_into_pairs;

lazy_static::lazy_static! {
    pub static ref MAP: Mutex<Vec<Vec<i32>>> = Mutex::new(
        vec![
            vec![10, 19, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
            vec![10, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 09, 10, 00, 00, 00, 00, 00, 09, 10],
            vec![10, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 10, 00, 00, 10, 00, 10],
            vec![10, 00, 00, 10, 10, 10, 10, 10, 10, 10, 10, 10, 00, 10, 10, 00, 00, 10, 00, 10],
            vec![10, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 10, 00, 10],
            vec![10, 00, 00, 00, 10, 10, 10, 10, 10, 10, 10, 00, 00, 00, 00, 00, 00, 10, 00, 10],
            vec![10, 00, 00, 00, 10, 09, 00, 00, 00, 00, 00, 00, 00, 10, 00, 00, 00, 10, 00, 10],
            vec![10, 10, 10, 00, 00, 19, 10, 10, 10, 10, 10, 00, 10, 10, 00, 00, 00, 10, 00, 10],
            vec![10, 00, 10, 00, 10, 00, 00, 00, 00, 00, 00, 00, 00, 10, 00, 00, 00, 10, 00, 10],
            vec![10, 00, 10, 00, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 00, 00, 00, 10, 00, 10],
            vec![10, 00, 10, 00, 00, 00, 00, 00, 00, 00, 10, 00, 00, 00, 10, 00, 00, 00, 00, 10],
            vec![10, 00, 00, 00, 00, 00, 00, 00, 00, 00, 10, 00, 10, 00, 00, 00, 00, 00, 00, 10],
            vec![10, 00, 09, 10, 10, 10, 10, 00, 00, 00, 10, 00, 10, 00, 10, 10, 10, 10, 00, 10],
            vec![10, 00, 10, 10, 09, 00, 10, 10, 10, 00, 10, 00, 10, 00, 10, 00, 00, 00, 00, 10],
            vec![10, 00, 00, 10, 00, 00, 00, 00, 10, 00, 00, 00, 10, 10, 10, 00, 00, 00, 00, 10],
            vec![10, 00, 00, 10, 00, 00, 00, 00, 10, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 10],
            vec![10, 00, 00, 10, 00, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 00, 10, 19, 10],
            vec![10, 00, 00, 00, 00, 00, 00, 00, 00, 09, 10, 09, 00, 00, 00, 00, 00, 00, 00, 10],
            vec![10, 09, 00, 00, 00, 00, 10, 00, 00, 00, 10, 09, 00, 00, 00, 00, 00, 00, 09, 10],
            vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]
        ]
    );
}

pub fn get_map() -> Vec<Vec<i32>> {
    MAP.lock().unwrap().clone()
}

pub fn set_value(x: usize, y: usize, value: i32) {
    let mut map = MAP.lock().unwrap();
    if let Some(row) = map.get_mut(y) {
        if let Some(cell) = row.get_mut(x) {
            *cell = value;
        }
    }
}

pub fn get_value(x: usize, y: usize) -> Option<i32> {
    let map = MAP.lock().unwrap();
    map.get(y).and_then(|row| row.get(x).copied())
}

pub fn calc_shadows() {
    let mut has_changes = true;
    let mut iter = 0;
    while has_changes {
        has_changes = false;
        let map = get_map();
        for (iter_y, row) in map.iter().enumerate() {
            for (iter_x, &cell) in row.iter().enumerate() {
                let mut pairs = split_into_pairs(cell);
                if pairs.len() > 1 {
                    pairs[0] = pairs[1]; // si es 9 se toma 9 pero si es 19 se toma 9, se salta el primer digito
                }
                let neighbors = [
                    (iter_x.wrapping_sub(1), iter_y),
                    (iter_x + 1, iter_y),
                    (iter_x, iter_y.wrapping_sub(1)),
                    (iter_x, iter_y + 1),
                ];
                for &(nx, ny) in &neighbors {
                    if let Some(value) = get_value(nx, ny) {
                        let new_value = (if value == 10 { 10 } else { 0 }) + pairs[0] - 1;
                        if new_value > value {
                            has_changes = true;
                            set_value(nx, ny, new_value);
                        }
                    }
                }
            }
        }
        iter += 1;
        println!("Iteracion: {}", iter);
        if iter > 10000 {
            break;
        }
    }
}
