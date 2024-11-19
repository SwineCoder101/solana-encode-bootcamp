fn main() {
    println!("Welcome to FizzBuzz!");
    fizzbuzz();
    fizzbuzz_with_match();

    let (x_1,x_2) = find_vector_addition(vec![1, 2, 3, 4, 5], 9);

    println!("The indices are: {}, {}", x_1, x_2);

    let (y_1,y_2) = find_vector_addition_hashmap(vec![1, 2, 3, 4, 5], 9);

    println!("The indices from hashmap are: {}, {}", y_1, y_2);
}

fn fizzbuzz() {
    let mut count = 0;

    for i in 1..301 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("Fizz Buzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
        count += 1;
    }

    println!("FizzBuzz complete! with {} iterations", count);
}

fn fizzbuzz_with_match() {
    let mut count = 0;

    for i in 1..301 {
        match (i % 3, i % 5) {
            (0, 0) => println!("Fizz Buzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
        count += 1;
    }

    println!("FizzBuzz complete! with {} iterations", count);
}

fn find_vector_addition(vector: Vec<i32>, target: i32) -> (i32, i32) {
//     We have Vector of integers called nums and a target integer. Return the two indices that add up to the target value.

// Rules:
// There’s always one unique solution for each list.
// You can’t use the same number twice.
    println!("target: {}", target); 


    for i in 0..vector.len() {
        for j in i+1..vector.len() {
            if vector[i] + vector[j] == target {
                println!("{} + {} = {}", vector[i], vector[j], target);
                return (i as i32, j as i32);
            }
        }
    }

    return (-1, -1);   
}

fn find_vector_addition_hashmap(vector: Vec<i32>, target: i32) -> (i32, i32) {
    println!("target: {}", target); 

    let mut map = std::collections::HashMap::new();

    for i in 0..vector.len() {
        let complement = target - vector[i];
        if map.contains_key(&complement) {
            return (*map.get(&complement).unwrap() as i32, i as i32);
        }
        map.insert(vector[i], i);
    }

    return (-1, -1);   
}