
// store x and y as objects to be called later for use with data type int64 
struct GenerisTask {
    x: i64,
    y: i64,
}

// creates different functionality that can be called by the trait by other types and shared
// with each functionality stored as a function each taking in two inputs each of data types int64(meaning they can store values fron -2^(63) to 2^(63) - 1) and give the return value in int64
trait GenerisTaskFunctions {
    fn calculate_multiplicative_inverse(&mut self , a: i64, m: i64) -> i64;
    fn _calculate_gcd(&mut self, a: i64, b: i64) -> i64;
}

// it shows that we are implementing the method GenerisTaskFunctions on the object value GenerisTask
// with the each functions acting on the Taking input of the samr type as GenerisTask
impl GenerisTaskFunctions for GenerisTask {
    fn calculate_multiplicative_inverse(&mut self , a: i64, m: i64) -> i64 {
        // we declare a variable g of datatype int64 to call the functin _calculate_gcd
        let g: i64 = self._calculate_gcd(a, m);
        // we implement the if statement not equals g not eqauls to 1 then we take the modulus of object x with input m in the above function called taking the modulus of our answer again
        if !(g != 1) {
            return(self.x % m + m) % m;
        }
        // if the statement is not met this is printed in the terminal output
        panic!("multiplicative inverse does not esxist for a={} m={}", a, m)
    }
    // function that takes in two input of type int64 and gives output int64 to compute the greatest common divisor(HCF) of two numbers given

    fn _calculate_gcd(&mut self, a: i64, b: i64) -> i64 {
        // if the input a is 0 set x,y to 0, 1 and return the value of b 
        if a == 0 {
            self.x = 0;
            self.y = 1;
            return b;
        }

        // declared variable gcd and call function again recursively this time the input being b modulus a and a with new values declared for x, y as x1, y1
        let gcd = self._calculate_gcd(b %a, a);
        let x1 = self.x;
        let y1 = self.y;

        // save the new value of  as this and new value of y as that
        self.x = y1 - (b/a) * x1;
        self.y = x1;
        
        return gcd;

    }
}

// declares an implementation type for GenerisTask and declare a new function to call it again with default valuse 0,1 for x,y respectively
impl GenerisTask {
    fn new() -> GenerisTask{
        GenerisTask { x: 0, y: 1 }
    }
}

// main function that runs the code
fn main() {
    //declare a mutable variable know as task to call on the GenerisTask function new above to initialize x and y as 0, 1 respectively
    let mut task = GenerisTask::new();
    // calls the function calculate_multiplicative_inverse on task above with input values 3 and 11
    let result_1 = task.calculate_multiplicative_inverse(3, 11);
    // prints the output
    println!("Multiplicative Inverse of A={} M={} -> {}", 3, 11, result_1);

    // repeats what is above for two new values
    let result2 = task.calculate_multiplicative_inverse(10, 17);
    // prints the outpur
    println!("Multiplicative Inverse of A={} M={} -> {}", 10, 17, result2);
}