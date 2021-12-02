use std::fs;

/// Load file
pub fn load_file() -> anyhow::Result<Vec<u32>> {
    let contents = fs::read_to_string("resources/day1.txt")?;

    Ok(contents
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect())
}

pub fn first_part() -> anyhow::Result<u32> {
    let list = load_file()?;
    let result = list
        .iter()
        .zip(list.iter().skip(1))
        .fold(0u32, |counter, value| {
            if value.0 < value.1 {
                counter + 1
            } else {
                counter
            }
        });

    Ok(result)
}

pub fn second_part() -> anyhow::Result<u32> {
    let lines = load_file()?;
    let sums = lines
        .iter()
        .zip(lines.iter().skip(1))
        .zip(lines.iter().skip(2))
        .map(|((x, y), z)| x + y + z);

    let sum = sums.clone().zip(sums.skip(1)).fold(0u32, |counter, value| {
        if value.0 < value.1 {
            counter + 1
        } else {
            counter
        }
    });

    Ok(sum)
}
