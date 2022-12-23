use std::collections::VecDeque;

const ENCRYPTION_KEY: i64 = 811589153;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let numbers = lines
        .iter()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let mut encrypted: VecDeque<_> = numbers.iter().copied().enumerate().collect();

    decrypt(&mut encrypted);

    println!("Part 1 {}", sum_coords(encrypted.make_contiguous()));

    let mut real_encrypted: VecDeque<_> = numbers
        .iter()
        .copied()
        .map(|v| v * ENCRYPTION_KEY)
        .enumerate()
        .collect();

    for _ in 0..10 {
        decrypt(&mut real_encrypted);
    }

    println!("Part 2 {}", sum_coords(real_encrypted.make_contiguous()));

    Ok(())
}

fn decrypt(encrypted: &mut VecDeque<(usize, i64)>) {
    for i in 0..encrypted.len() {
        let current_i = encrypted.iter().position(|(j, _)| i == *j).unwrap();
        let current = encrypted.remove(current_i).unwrap();
        let next_i = if current.1 > 0 {
            let i = current_i + current.1 as usize;
            i % encrypted.len()
        } else {
            let mut i = (current_i as i64 + current.1) % encrypted.len() as i64;

            if i < 0 {
                i += encrypted.len() as i64;
            } else if i == 0 {
                i = encrypted.len() as i64;
            }

            i.try_into().unwrap()
        };
        encrypted.insert(next_i, current);
    }
}

fn sum_coords(decrypted: &[(usize, i64)]) -> i64 {
    let index = decrypted.iter().position(|(_, v)| *v == 0).unwrap();

    let first = decrypted[(index + 1000) % decrypted.len()].1;
    let second = decrypted[(index + 2000) % decrypted.len()].1;
    let third = decrypted[(index + 3000) % decrypted.len()].1;

    first + second + third
}
