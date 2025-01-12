use std::sync::Mutex;
use crate::utils::split_into_pairs;

lazy_static::lazy_static! {
    pub static ref MAP: Mutex<Vec<Vec<i32>>> = Mutex::new(
        vec![
            vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
            vec![10, 09, 00, 00, 00, 00, 00, 00, 00, 00, 00, 09, 10, 00, 00, 00, 00, 00, 09, 10],
            vec![10, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 10, 00, 00, 10, 00, 10],
            vec![10, 00, 00, 10, 10, 10, 10, 10, 10, 10, 10, 10, 00, 10, 10, 00, 00, 10, 00, 10],
            vec![10, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 10, 00, 10],
            vec![10, 00, 00, 00, 10, 10, 10, 10, 10, 10, 10, 00, 00, 00, 00, 00, 00, 10, 00, 10],
            vec![10, 00, 00, 00, 10, 09, 00, 00, 00, 00, 00, 00, 00, 10, 00, 00, 00, 10, 00, 10],
            vec![10, 10, 10, 00, 00, 10, 10, 10, 10, 10, 10, 00, 10, 10, 00, 00, 00, 10, 00, 10],
            vec![10, 00, 10, 00, 10, 00, 00, 00, 00, 00, 00, 00, 00, 10, 00, 00, 00, 10, 00, 10],
            vec![10, 00, 10, 00, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 00, 00, 00, 10, 00, 10],
            vec![10, 00, 10, 00, 00, 00, 00, 00, 00, 00, 10, 00, 00, 00, 10, 00, 00, 00, 00, 10],
            vec![10, 00, 00, 00, 00, 00, 00, 00, 00, 00, 10, 00, 10, 00, 00, 00, 00, 00, 00, 10],
            vec![10, 00, 00, 10, 10, 10, 10, 00, 00, 00, 10, 00, 10, 00, 10, 10, 10, 10, 00, 10],
            vec![10, 00, 10, 10, 00, 00, 10, 10, 10, 00, 10, 00, 10, 00, 10, 00, 00, 00, 00, 10],
            vec![10, 00, 00, 10, 00, 00, 00, 00, 10, 00, 00, 00, 10, 10, 10, 00, 00, 00, 00, 10],
            vec![10, 00, 00, 10, 00, 00, 00, 00, 10, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 10],
            vec![10, 00, 00, 10, 00, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 00, 10, 10, 10],
            vec![10, 00, 00, 00, 00, 00, 00, 00, 00, 00, 10, 00, 00, 00, 00, 00, 00, 00, 00, 10],
            vec![10, 09, 00, 00, 00, 00, 10, 00, 00, 00, 10, 00, 00, 00, 00, 00, 00, 00, 09, 10],
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
    // guardar para ahorrar calculo (?)
    let mut has_changes = true;
    let mut iter = 0;
    while has_changes {
        has_changes = false;
        let mut iter_x = 0;
        let mut iter_y = 0;
        for i in get_map().iter() {
            for j in i.iter() {
                let pairs = split_into_pairs(*j);
                if pairs[0] < 10 && pairs[0] > 0 && *j < 10 {
                    // *j < 10 (verifica que tenga 2 digitos)
                    // println!("{:?}", pairs);
                    // println!("{} - {}", iter_x, iter_y);

                    if let Some(value) = get_value(iter_x - 1, iter_y) {
                        let new_value = (if value == 10 { 10 } else { 0 }) + pairs[0] - 1;
                        if new_value > value {
                            has_changes = true;
                            set_value(iter_x - 1, iter_y, new_value);
                        }
                    }
                    if let Some(value) = get_value(iter_x + 1, iter_y) {
                        let new_value = (if value == 10 { 10 } else { 0 }) + pairs[0] - 1;
                        if new_value > value {
                            has_changes = true;
                            set_value(iter_x + 1, iter_y, new_value);
                        }
                    }
                    if let Some(value) = get_value(iter_x, iter_y - 1) {
                        let new_value = (if value == 10 { 10 } else { 0 }) + pairs[0] - 1;
                        if new_value > value {
                            has_changes = true;
                            set_value(iter_x, iter_y - 1, new_value);
                        }
                    }
                    if let Some(value) = get_value(iter_x, iter_y + 1) {
                        let new_value = (if value == 10 { 10 } else { 0 }) + pairs[0] - 1;
                        if new_value > value {
                            has_changes = true;
                            set_value(iter_x, iter_y + 1, new_value);
                        }
                    }
                }
                iter_y += 1;
            }
            iter_x += 1;
            iter_y = 0;
        }
        iter += 1;
        println!("Iteracion: {}", iter);
        if iter > 10000 {
            break;
        }
    }
}
