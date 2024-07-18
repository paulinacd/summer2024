fn fahrenheit_to_celsius(f: f64) -> f64{
    //Converts Fahrenheit to Celsius
    (f - (FREEZING_POINT as f64)) * 5.0/9.0 as f64
        
    }
    
    fn celsius_to_fahrenheit(c: f64) -> f64{
    c + 10.0
        
    }
    
    
        const FREEZING_POINT: f32 = 32.0;
    
    fn main(){
     let mut fahrenheit = 100.0;
     let in_celsius = fahrenheit_to_celsius(fahrenheit);
     
     let temperatures = [33, 34, 35, 36, 37];
     for i in 0..temperatures.len(){
    println!("{}", fahrenheit_to_celsius(temperatures[i] as f64));
     }
    }