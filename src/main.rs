use std::io;
use rand::Rng;
use std::process;
mod utils;

pub struct Guess{
    series: [i32;5],
    answer: i32
}


impl Guess{
    fn new() -> Guess{
        /*
            the constructor
        */
        Guess{
            series: [0,1,2,3,4],
            answer: 0
        }
    }
    fn compare(&self,answer: i32) -> bool{
        /*
            compare the user answer with the program generated answer
            input:
                answer: the user's answer
                self: the series object with the program generated answer
        */
        if self.answer == answer {
            return true
        }
        else{
            return false
        }
    }
    fn gp_fill(&mut self,a: i32, r: i32) {
        /*
            creates a geometric series
        */
        println!("creating an geometric series");
        for i in 0..6{
            if i < 5{
                self.series[i as usize] = a * r.pow(i);
            }
            else{
                self.answer = a * r.pow(i);
            }
        }
    }
    fn ar_fill(&mut self, a: i32, d:i32) {
        /*
            creates an arithmetic series
        */     
        println!("creating an arithmetic series");   
        for i in 0..6{
            if i < 5{
                self.series[i as usize] = a + i*d;
            }
            else{
                self.answer = a + i * d;
            }
            
        }
    }
    fn fill_series(&mut self ,x: i32) -> bool{
        /*
            fill the series according to the given type
            Inputs:
            x: the type of series, 1: arithmetic, 2: geometric
        */
        let mut rng = rand::thread_rng();
        let a: i32 = rng.gen_range(2, 30);
        let r: i32 = rng.gen_range(2, 10);
        if x == 1 {
            self.ar_fill(a,r);
            return true;
        }
        else if x == 2 {
            self.gp_fill(a,r);
            return true;
        }
        else{
            println!("parhna likhna seekh kar ah bhai!!! 1 ya 2 enter karna hai");
            return false;
        }
    }
}

fn main() {
    // declare variables
    let mut guesstr = String::new();
    let mut secret = Guess::new();
    // ask user for type of series 
    println!("select series type: types:\n 1: Arithmetic\n 2: Geometric");
    io::stdin()
        .read_line(&mut guesstr)
        .expect("Failed to read line");
    let x: i32 = guesstr.trim().parse().expect("what sort of bullshit u feeding bro");
    // generate series 
    let cont = secret.fill_series(x);
    if cont == false{
        process::exit(0x0100);
    }
    guesstr = "".to_string();

    // print series
    println!("The series: {:?}\nGuess the next number Ma Genius Baby:",secret.series);
    
    //Taking user input 
    io::stdin()
        .read_line(&mut guesstr)
        .expect("What sort of bullshit u feeding???");

    let x: i32 = guesstr.trim().parse().expect("Input was not a number");
    //compare the answer to the input 
    if secret.compare(x) {
        println!("Ya Ya you guessed right, u little punk ");
    }
    else{
        println!("fuck off! U illiterate FUCK!!!! \nthe correct answer is: {}",secret.answer);
    }
}

