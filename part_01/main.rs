fn sqr(i: i32) -> i32 {
    i * i
}

fn abs(i: i32) -> i32 {
    if i >= 0   {i}
    else        {-i}
}

enum NumberOrNothing {
    Number(i32),
    Nothing
}
use self::NumberOrNothing::{Number, Nothing};

fn number_or_default(n: NumberOrNothing, default: i32) -> i32 
{
    match n {
        Nothing => default,
        Number(n) => n,
    }
}

// returns the min of a vec32
fn vec_min(v: Vec<i32>) ->NumberOrNothing {
    fn min_i32(a: i32, b: i32) -> i32 {
        if a > b    { a }
        else        { b }
    }
    let mut min = Nothing;
    for e in v {
        min = Number(match min {
        Nothing => e,
        Number(n) => min_i32(n, e)
        });
    }
    min
}

fn read_vec() -> Vec<i32> {
    vec![18,5,7,2,9,27]
}

impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Number(n) =>println!("The number is: {}", n),
        };
    }
}

fn vec_print(v: Vec<i32>) {
    for e in v {
        println!("elem {}", e);
    }
}

fn vec_sum(v: Vec<i32>) -> i32 {
    let mut min = 0;
    for e in v {
        min += e;
    }
    min
}

pub fn main()
{
    let i = 32;
    let str = "hello";
    println!("The number is: {}", i);
    println!("The str is: {}", str);
    let n = number_or_default(NumberOrNothing::Number(i), 42);
    println!("The number as enum is: {}", n);
    let n = number_or_default(NumberOrNothing::Nothing, 42);
    println!("The str as enum is: {}", n);
    let i = -2;
    let n = sqr(i);
    println!("The number i {} sqrd is: {}", i, n);
    let i = -2;
    let n = abs(i);
    println!("The number i {} abs is: {}", i, n);
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
    let vec = read_vec();
    vec_print(vec);
    let vec = read_vec();
    let sum = vec_sum(vec);
    println!("The sum of vec is {}", sum);
}
