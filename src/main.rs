use rand::Rng;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{encode, validate};

    #[test]
    fn validation_test() {
        for _i in 0..1000 {
            let num_disc_total: usize = rand::thread_rng().gen_range(3..50);
            let num_disc_recovery: usize = f64::log((num_disc_total + 1) as f64, 2.0).ceil() as usize;
            let num_buff: usize = rand::thread_rng().gen_range(1..100);
    
            let mut disks: Vec<Vec<i32>> = vec![];
            let mut disk: Vec<i32>;
            for _i in 0..num_disc_total {
                disk = vec![0; num_buff];
                for j in 0..disk.len() {
                    disk[j] = rand::thread_rng().gen_range(0..2);
                }
                disks.append(&mut vec![disk]);
            }
    
            encode(&mut disks, num_disc_recovery, num_buff);
            let valid = disks.clone();

            let mut error_pos_x: usize;
            let mut error_pos_y: usize;
            loop {
                error_pos_x = rand::thread_rng().gen_range(0..num_disc_total);
                error_pos_y = rand::thread_rng().gen_range(0..num_buff);

                // checks if we make error in Hamming code`s index
                if error_pos_x.count_ones() != 0 && error_pos_x != 0 {
                    break;
                }
            }

            disks[error_pos_x][error_pos_y] = (disks[error_pos_x][error_pos_y] - 1).abs();

            validate(&mut disks, num_disc_recovery, num_buff);
    
            assert_eq!(disks, valid);
        } 

    }
}

fn main() {
    let num_disc_total: usize = 7;
    let num_disc_recovery: usize = f64::log((num_disc_total + 1) as f64, 2.0).ceil() as usize;
    let num_buff: usize = 1;
    
    let mut disks: Vec<Vec<i32>> = vec![];

    let mut disk: Vec<i32>;
    for _i in 0..num_disc_total {
        disk = vec![0; num_buff];
        for j in 0..disk.len() {
            disk[j] = rand::thread_rng().gen_range(0..2);
        }
        disks.append(&mut vec![disk]);
    }

    encode(&mut disks, num_disc_recovery, num_buff);
    print_disks_info(&disks);

    let try_write: Vec<i32> = vec![1, 1, 1, 1];
    write(&mut disks, 0, try_write);
    print_disks_info(&disks);
}

fn print_disks_info(disks: &Vec<Vec<i32>>) {
    for i in 0..disks[0].len() {
        for j in 0..disks.len() {
            print!("{}", disks[j][i])
        }
        println!();
    }
}

// applies Hamming code for disks
fn encode(disks: &mut Vec<Vec<i32>>, num_disc_recovery: usize, num_buff: usize) {
    let mut index: i32;
    for j in 0..num_buff {
        for i in 0..num_disc_recovery {
            index = i32::pow(2, i as u32) - 1;
            disks[index as usize][j as usize] = calculate_code(disks, index, j);
        }
    }
}

// checks mistakes and correct them
fn validate(disks: &mut Vec<Vec<i32>>, num_disc_recovery: usize, num_buff: usize) {
    let mut index: i32;
    let mut error_index: i32;
    for j in 0..num_buff {     
        error_index = 0;   
        for i in 0..num_disc_recovery {
            index = i32::pow(2, i as u32) - 1;
            let res: i32 = calculate_code(disks, index, j);
            if res != disks[index as usize][j] {
                error_index += index + 1;
            }
        }
        if error_index != 0 {
            disks[(error_index - 1) as usize][j] = (disks[(error_index - 1) as usize][j] - 1).abs();
        }
    }
}

// calculates Hamming code for certain position
fn calculate_code(disks: &mut Vec<Vec<i32>>, index: i32, j: usize) -> i32 {
    let mut res: i32 = 0;
    let mut c: i32 = 0;
    let mut k: i32 = index;
    while k < disks.len() as i32 {
        if index == k { c += 1; k += 1; continue; }
        while c < index + 1 {
            if k >= disks.len() as i32 { break; }
            if (k + 1).count_ones() != 1 { res += disks[k as usize][j]; }
            c += 1;
            k += 1;
        }
        while c > 0 {
            c -= 1;
            k += 1;
        }
    }
    res % 2
}

fn write(disks: &mut Vec<Vec<i32>>, segment: usize, info: Vec<i32>) {
    if info.len() != disks.len() - (f64::log((disks.len() + 1) as f64, 2.0).ceil() as usize) {
        println!("Unable to write");
        return;
    }
    
    let mut c: usize = 0;
    for i in 0..disks.len() {
        if (i + 1).count_ones() == 1 { continue; }
        disks[i][segment] = info[c];
        c += 1;
    };

    encode(disks, f64::log((disks.len() + 1) as f64, 2.0).ceil() as usize, disks[0].len())
}

fn read(disks: &Vec<Vec<i32>>, segment: usize) -> Vec<i32> {
    if segment >= disks[0].len() {
        println!("out of bounds");
        return vec![];
    }

    let mut res: Vec<i32> = vec![0; disks.len()];
    for i in 0..disks.len() {
        res[i] = disks[i][segment];
    };

    return res;
}

fn delete(disks: &mut Vec<Vec<i32>>, segment: usize) {
    if segment >= disks[0].len() {
        println!("out of bounds");
        return;
    }

    for i in 0..disks.len() {
        disks[i].remove(segment);
    };
}