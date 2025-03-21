use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    let result  = Arc::new(Mutex::new(HashMap::new()));
    let input_vec: Vec<String> = input.iter().map(|s| s.to_string().to_lowercase()).collect();
    let input_arc = Arc::new(input_vec);


    let handlers: Vec<_> = (0..worker_count).map(|worker_id|{
        let result = Arc::clone(&result);
        let input_arc = Arc::clone(&input_arc);
        spawn( move || {
            input_arc.iter().enumerate().for_each(|(index,line)|{
                if index % worker_count != worker_id{
                    return;
                }
                line.chars().for_each(|c|{
                    if !c.is_alphabetic(){
                        return;
                    }
                    let mut map = result.lock().unwrap();
                    let counter = map.entry(c).or_insert(0);
                    *counter += 1;
                });
            });
           
        })
    }).collect::<Vec<_>>();

    for handler in handlers {
        handler.join().unwrap();
    }
    
    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}
