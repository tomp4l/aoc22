pub fn run(lines: Vec<String>) -> Result<(), String> {
    let sum: i64 = lines.iter().map(|l| snafu_to_i64(&l)).sum();

    println!("Part 1 {}", i64_to_snafu(sum));

    Ok(())
}

fn snafu_to_i64(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            '=' => -2 * pow5(i),
            '-' => -1 * pow5(i),
            '0' => 0,
            '1' => 1 * pow5(i),
            '2' => 2 * pow5(i),
            c => panic!("unexpected {}", c),
        })
        .sum()
}

fn i64_to_snafu(i: i64) -> String {
    let mut remainder = i;

    let mut chars = Vec::new();

    let mut i = 1;
    while pow5(i) - 2 * sum_pow5(i - 1) <= remainder {
        i += 1;
    }

    for i in (1..i).rev() {
        if remainder >= 0 {
            let x = (remainder + 2 * sum_pow5(i - 1)) / pow5(i);

            remainder -= x * pow5(i);
            chars.push(match x {
                2 => '2',
                1 => '1',
                0 => '0',
                _ => unreachable!(),
            });
        } else {
            let n = remainder + 2 * sum_pow5(i - 1);
            if n >= 0 {
                chars.push('0');
            } else {
                let x = (n + 1) / pow5(i);
                remainder -= (x - 1) * pow5(i);
                chars.push(match x {
                    0 => '-',
                    -1 => '=',
                    _ => unreachable!(),
                });
            }
        }
    }

    chars.push(match remainder {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => unreachable!(),
    });

    chars.iter().collect()
}

fn pow5(i: usize) -> i64 {
    let mut x = 1;
    if i == 0 {
        x
    } else {
        for _ in 0..i {
            x *= 5;
        }
        x
    }
}

fn sum_pow5(i: usize) -> i64 {
    let mut sum = 0;
    for i in 0..i + 1 {
        sum += pow5(i);
    }
    sum
}
