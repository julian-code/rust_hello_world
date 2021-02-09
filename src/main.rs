use std::mem;

const MEANING_OF_LIFE:u8 = 42; // no fixed address

static Z:i32 = 123; //adding mut will force you to wrap every use of Z in a unsafe block. Not recommended. 

fn main() {
    println!("fundamental data types");
    fundamental_data_types();
    println!("operators");
    operators();
    println!("scope and shadowing");
    scope_and_shadowing();

    println!("meaning of life {}", MEANING_OF_LIFE);
    println!("static value {}", Z);
}

fn scope_and_shadowing() {
    let a = 123;
    let a = 1234; //overrides the above variable no problem. 

    {
        let b = 456;
        let a = 777; //shadowing the "inside" a variable
        println!("inside, b = {}", b);
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
    //println!("outside, b = {}", b) compile error!
}

fn fundamental_data_types() {
    // unsigned integer 0..255, size of one byte
    let a:u8 = 123;
    //signed integer -128..127, size of one byte
    let b:i8 = 123;
    println!("a = {}", a);

    //mut keyword makes a value mutable. immutable by default!
    let mut c:i8 = 0;
    println!("c = {}", c);
    c = 42;
    println!("c = {}", c);

    let mut d = 123456789; // 32-bit signed i32, size of 4 bytes
    //lets prove it!
    println!("c = {}, size = {} bytes", d, mem::size_of_val(&d));
    d = -1;
    println!("c = {} after mutation", d);

    // i8 u8 i16 u16 i32 u32 i64 u64 is possible!
    let z:isize = 123; // isize/usize - a pointer with the size of a mem address.
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    //largest unicode size, 4 bytes
    let e = 'x';
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // double precision, 8 bytes / 64 bits, f64 (floating point)
    let f = 2.5;
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

    // bool values: true or false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

fn operators() 
{
    //arithmetics

    let mut a = 2+3*4; // +-*/ all follow precedence

    println!("{}", a);
    a = a+1; // does not support -- ++
    a -= 2; // a = a - 2;

    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    //integer as the power
    let b_cubed = f64::powi(b, 3);
    //floating numbers as power
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    //bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                   // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10;

    println!("2^10 = {}", two_to_10);
    
    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!("pi is less than 4 {}", pi_less_4);
    // > <= >=
    let x = 5;
    let x_is_5 = x == 5; // true
    println!("x is equal to 5 {}", x_is_5);
}

