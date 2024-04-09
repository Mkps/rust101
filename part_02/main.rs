pub enum SomethingOrNothing<T>
{
    Something(T),
    Nothing,
}

pub use self::SomethingOrNothing::*;

type NumberOrNothing = SomethingOrNothing<i32>;
type FloatOrNothing = SomethingOrNothing<f32>;

impl<T> SomethingOrNothing<T>
{
    fn new(o: Option<T>) -> Self
    {
        match o { None => Nothing, Some(t) => Something(t) }
    }
    fn to_option(self) -> Option<T>
    {
        match self  { Nothing => None, Something(t) => Some(t) }
    }
}

fn call_constructor_i32(x: i32) -> SomethingOrNothing<i32>
{
    SomethingOrNothing::new(Some(x))
}

fn call_constructor_f32(x: f32) -> SomethingOrNothing<f32>
{
    SomethingOrNothing::new(Some(x))
}

pub trait Minimum : Copy
{
    fn min (self, b: Self) -> Self;
}

pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T>
{
    let mut min = Nothing;
    for e in v
    {
        min = Something(match min
                        {
                            Nothing => e,
                            Something(n) => { e.min(n) }
                        });
    }
    min
}

impl Minimum for i32
{
    fn min(self, b: Self) -> Self
    {
        if self < b { self }
        else        { b }
    }
}

impl Minimum for f32
{
    fn min(self, b: Self) -> Self
    {
        if self < b { self }
        else        { b }
    }
}


impl NumberOrNothing
{
    pub fn print(self)
    {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}

impl FloatOrNothing
{
    pub fn print(self)
    {
        match self
        {
            Nothing => println!("The float is: <nothing>"),
            Something(n) => println!("The f32 number is: {}", n),
        };
    }
}

fn read_vec() -> Vec<i32>
{
    vec![18,5,7,3,9,27]
}

fn read_fvec() -> Vec<f32>
{
    vec![0.5, 1.5, 4.2, 2.0, 1.8]
}

pub fn main()
{
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
    let f32_vec = read_fvec();
    let f32_min = vec_min(f32_vec);
    f32_min.print();
}
