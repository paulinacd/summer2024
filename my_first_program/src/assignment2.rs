fn is_even(n: i32) -> bool{
    if n % 2 == 0{
        true
    }
    else{
        false
    }
}


fn main(){
    let array = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
for i in 0..array.len(){
    if array[i] % 3 == 0 && array[i] % 5 == 0{
        print!("{} FizzBuzz ", array[i]);
    }
    else if array[i] % 3 == 0{
        print!("{} Fizz " , array[i]);
    }
    else if array[i] % 5 == 0{
        print!("{} Buzz ", array[i]);
    }
    println!("{}", is_even(array[i]));
}
let mut counter = array.len()-1;
let mut sum = array[0];
while counter != 0{
     sum += array[counter];
    counter-=1;
}
println!("{}", sum);
let mut largest = array[0];
let mut i = 0;
loop {
    if array[i] > largest{
        largest = array[i];
    }
    i+=1;
    if i == array.len()  {
        break;
    }
}
println!("{}", largest);
}