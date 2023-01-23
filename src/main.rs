#![recursion_limit = "512"]
#![allow(dead_code)]

#[derive(Debug)]
/// An empty enum, a type without inhabitants.
/// Cf: https://en.wikipedia.org/wiki/Bottom_type
enum Null {}

/// PeanoEncoder<Null> is a Peano-type able to represent integers up to 0.
/// If T is a Peano-type able to represent integers up to n
/// PeanoEncoder<T> is a Peano-type able to represent integers up to n+1
#[derive(Debug)]
enum PeanoEncoder<T> {
    Successor(T),
    Zero,
}

macro_rules! times2 {
    ($peano_2x:ident, $peano_x:ident ) => {
        type $peano_2x<T> = $peano_x<$peano_x<T>>;
    };
}
times2!(PeanoEncoder2, PeanoEncoder);
times2!(PeanoEncoder4, PeanoEncoder2);
times2!(PeanoEncoder8, PeanoEncoder4);
times2!(PeanoEncoder16, PeanoEncoder8);
times2!(PeanoEncoder32, PeanoEncoder16);
times2!(PeanoEncoder64, PeanoEncoder32);
times2!(PeanoEncoder128, PeanoEncoder64);
times2!(PeanoEncoder256, PeanoEncoder128);

type Peano0 = PeanoEncoder<Null>;
type Peano256 = PeanoEncoder256<Null>;

impl Peano256 {
    // This should be unsafe
    pub fn transmute_u8(x: u8) -> Self {
        unsafe { std::mem::transmute(u8::MAX - x) }
    }
}

trait IntoU8 {
    fn into_u8(self) -> u8;
}

impl IntoU8 for Null {
    fn into_u8(self) -> u8 {
        match self {}
    }
}

impl<T: IntoU8> IntoU8 for PeanoEncoder<T> {
    fn into_u8(self) -> u8 {
        match self {
            PeanoEncoder::Successor(x) => 1 + x.into_u8(),
            PeanoEncoder::Zero => 0,
        }
    }
}

fn main() {
    println!(
        "Size of Peano256: {} byte",
        std::mem::size_of::<Peano256>()
    );
    for i in 0_u8..=8 {
        let x = Peano256::transmute_u8(i);
        println!("transmute(u8::MAX - {}) = {:?}", i, x);
    }
    for i in 0_u8..=u8::MAX {
        let x = Peano256::transmute_u8(i);
        if i % 8 == 0 {
            print!("{:3} ", i)
        } else if i%8 == 4 {
            print!(" ")
        }
        let c = if x.into_u8() == i { '✓' } else { '✗' };
        print!("{}", c);
        if i % 8 == 7 {
            println!()
        }
    }
}
