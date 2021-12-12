use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    //println!("Hello, world!");

    //combined two var into a hashmap
    let teams = vec![String::from("Red"), String::from("Black")];
    let initial_scores = vec![20, 30];
    let mut scores2: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();


        //access values
        let team_name = String::from("Red");
        println!("Red team has score of {:?}", scores2.get(&team_name));
        match scores2.get(&team_name) {
            Some(score) => println!("Red has {}", score),
            None => println!("No value")
        }

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        //overwriting value
        scores.insert(String::from("Blue"), 60);

        //insert if no entry
        scores.entry(String::from("Blue")).or_insert(40);
        scores.entry(String::from("Green")).or_insert(40);
        println!("{:?}", scores);

        //update value based on old value
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);

}
