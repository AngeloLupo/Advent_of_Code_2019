use std::fs;

fn main() {
    println!("One: {}", one());
    println!("One_two: {}", one_two());
    println!("Two: {}", two());
    println!("Two_two: {}", two_two());
}

// Day one

fn one() -> i32 {
    let input = fs::read_to_string("src/inputs/one.txt").expect("Unable to read file");
    let modules = input.split("\n");

    let mut fuel_required: f64 = 0.0;
    for module in modules {
        let int_module: i32 = module.parse().unwrap();
        fuel_required += ((int_module/3) as f64).floor()-(2 as f64);
    }

    return fuel_required as i32;
}

fn one_two() -> i64 {
    let input = fs::read_to_string("src/inputs/one.txt").expect("Unable to read file");
    let modules = input.split("\n");

    let mut total_fuel_required: i64 = 0;
    for module in modules {
        let int_module: i64 = module.parse().unwrap();
        let mut fuel = int_module;

        while fuel > 0 {
            fuel = (((fuel/3) as f64).floor()-(2 as f64)) as i64;
            if fuel > 0 {
                total_fuel_required += fuel;
            }
        }
    }

    total_fuel_required as i64
}

// Day Two

fn two() -> i32 {
    let input = fs::read_to_string("src/inputs/two.txt").expect("Unable to read file");
    let mut opcodes: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    opcodes[1] = 12;
    opcodes[2] = 2;

    let mut position: usize = 0;
    let items = opcodes.len() - 1;

    loop {
        if position > items {
            break;
        }

        match opcodes[position] {
            1 => {
                let position_1 = opcodes[position+1 as usize] as usize;
                let position_2 = opcodes[position+2 as usize] as usize;
                let position_3 = opcodes[position+3 as usize] as usize;

                opcodes[position_3] = opcodes[position_1] + opcodes[position_2];
                position += 4;
            },
            2 => {
                let position_1 = opcodes[position+1 as usize] as usize;
                let position_2 = opcodes[position+2 as usize] as usize;
                let position_3 = opcodes[position+3 as usize] as usize;

                opcodes[position_3] = opcodes[position_1] * opcodes[position_2];
                position += 4;
            },
            99 => break,
            _ => println!("weird instruction"),
        }
    }
    return opcodes[0] as i32;
}

fn two_two() -> i32 {
    let input = fs::read_to_string("src/inputs/two.txt").expect("Unable to read file");
    let original_opcodes: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    loop {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut opcodes = original_opcodes.clone();
                opcodes[1] = noun;
                opcodes[2] = verb;
                let mut position: usize = 0;
                let items = opcodes.len() - 1;

                loop {
                    if position > items {
                        break;
                    }

                    match opcodes[position] {
                        1 => {
                            let position_1 = opcodes[position+1 as usize] as usize;
                            let position_2 = opcodes[position+2 as usize] as usize;
                            let position_3 = opcodes[position+3 as usize] as usize;

                            opcodes[position_3] = opcodes[position_1] + opcodes[position_2];
                            position += 4;
                        },
                        2 => {
                            let position_1 = opcodes[position+1 as usize] as usize;
                            let position_2 = opcodes[position+2 as usize] as usize;
                            let position_3 = opcodes[position+3 as usize] as usize;

                            opcodes[position_3] = opcodes[position_1] * opcodes[position_2];
                            position += 4;
                        },
                        99 => break,
                        _ => println!("weird instruction"),
                    }
                }
                if opcodes[0] == 19690720 {
                    return (100 * noun) + verb;
                }
            }
        }
    }
}