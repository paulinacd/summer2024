fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        0
    }
    else if guess < secret {
        -1
    }
    else {
        1
    }
}
fn main(){
    let secret = 33;
    let guesses = [11, 22, 33, 44, 55];
let mut g = 1;
let mut h = 0;
    loop{
    let mut guess = guesses[h];
    print!("Guess: {} ", guess);
    if check_guess(guess, secret) == 0{
        println!("Correct");
        break;
    }
    else if check_guess(guess, secret) == -1{
        println!("Too low");
    } 
    else{
        println!("Too high");
    }
    g+=1;
    h+=1;
}
println!("Number of guesses: {}", g);
}