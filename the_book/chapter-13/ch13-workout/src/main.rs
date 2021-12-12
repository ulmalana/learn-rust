use std::thread;
use std::time::Duration;

struct Cacher<T> 
where 
    T: Fn(u32) -> u32, 
{
        calculation: T,
        value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn simulated_expensive_calc(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, rand_num: u32) {
    //let expensive_result = simulated_expensive_calc(intensity);

    // //closure
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    //closure with struct for memoization
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    }
    else {
        if rand_num == 3 {
            println!("Take a break today. Stay hydrated.");
        }
        else {
            println!("Today, run for {} minutes", expensive_result.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_value = 10;
    let simulated_rand_num = 3;

    generate_workout(simulated_user_value, simulated_rand_num);
    //println!("Hello, world!");
}
