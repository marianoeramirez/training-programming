use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::collections::HashSet;
use std::io::{stdin,stdout,Write};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Rate{
    initial_coin: String,
    destination_coin: String,
}

fn find_conversion_path(initial_coin:String , destination_coin:String, conversation_rate: &HashMap<Rate, f64>) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut queue: Vec<Vec<String>> = vec![];
    queue.push(vec![initial_coin.clone()]);

    while !queue.is_empty() {
        let path = queue.remove(0);

        let current_coin = path.last().unwrap().clone();

        if current_coin == destination_coin {
            return path;
        }
        let neighbours = conversation_rate.iter()
            .filter(|(rate, _)| rate.initial_coin == current_coin)
            .map(|(rate, _)| rate.destination_coin.clone())
            .collect::<Vec<_>>();
        for neighbour in neighbours {
            if !visited.contains(&neighbour) {
                let mut new_path = path.clone();
                new_path.push(neighbour.clone());
                queue.push(new_path);                
                visited.insert(neighbour);
                dbg!(&queue);
            }
        }
    }
    vec![]
}

fn calculate_path(path: Vec<String>, convertion_rate: &HashMap<Rate, f64>) -> f64 {
    let mut total = 1.0;
    for i in 0..path.len()-1{
        let rate = Rate{initial_coin: path[i].clone(), destination_coin: path[i+1].clone()};
        total *= convertion_rate.get(&rate).unwrap();
    }
    total
}

fn main() {
    let mut file = File::open("conversion_rate.csv").unwrap();
    let mut contents = String::new();

    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file: {}", e);
        return;
    }

    let mut conversation_rate: HashMap<Rate, f64>  = HashMap::new();

    contents.split("\r\n").for_each(|line| {
        let items: Vec<_> = line.split(' ').collect();
        println!("{:?}", items[2]);
        let num = items[2].parse::<f64>().unwrap();

        conversation_rate.insert(Rate{ 
            initial_coin:items[0].into(), 
            destination_coin:items[1].into()}, 
            num);
        conversation_rate.insert(Rate{ 
            initial_coin:items[1].into(), 
            destination_coin:items[0].into()}, 
            1.0/num);
    });


    println!("{:#?}", conversation_rate);

    let mut s=String::new();
    print!("Please enter coin desired: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string?");
    let conversion_input: Vec<_> = s.split(" ").collect();
    
    let path = find_conversion_path(conversion_input[0].trim().to_string(), conversion_input[1].trim().to_string(), &conversation_rate);
    println!("{:?}", path);

    let total = calculate_path(path, &conversation_rate);
    println!("Total: {}", total);

}


// Case 1 
        // // EUR GBP 0.75
        // // EUR USD 1.17

        // let existing_rates: HashMap<Rate, f64>  = conversation_rate.iter()
        //     .filter(|(rate, _)| rate.initial_coin == items[0] && rate.destination_coin != items[1])
        //     .map(|(rate, &number)| (rate.clone(), number))
        //     .collect();

        // for (rate, number) in existing_rates {
        //     conversation_rate.insert(Rate{ 
        //         initial_coin:rate.destination_coin.into(), 
        //         destination_coin:items[1].into()}, 
        //         num * number);
        // }

        // // Case 2 
        // // GBP EUR 0.75
        // // EUR USD 1.17

        // let existing_rates: HashMap<Rate, f64>  = conversation_rate.iter()
        //     .filter(|(rate, _)| rate.destination_coin == items[0] && rate.initial_coin != items[1])
        //     .map(|(rate, &number)| (rate.clone(), number))
        //     .collect();

        // for (rate, number) in existing_rates {
        //     conversation_rate.insert(Rate{ 
        //         initial_coin:rate.initial_coin.into(), 
        //         destination_coin:items[0].into()}, 
        //         num * number);
        // }

