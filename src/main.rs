#![recursion_limit = "512"]
#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
/// An empty enum, a type without inhabitants.
/// Cf: https://en.wikipedia.org/wiki/Bottom_type
enum Null {}

/// PeanoEncoder<Null> is a Peano-type able to represent integers up to 0.
/// If T is a Peano-type able to represent integers up to n
/// PeanoEncoder<T> is a Peano-type able to represent integers up to n+1
#[derive(Debug, Clone, Copy)]
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
type Peano1 = PeanoEncoder<Peano0>;
type Peano2 = PeanoEncoder<Peano1>;
type Peano255 = PeanoEncoder256<Null>;

impl Peano255 {
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

/// No discriminant ellision for this one!
enum Bin<T> {
    A(T),
    B(T),
}

fn main() {
    println!("Size of Peano255: {} byte", std::mem::size_of::<Peano255>());
    for x in [
        Peano255::Zero,
        Peano255::Successor(PeanoEncoder::Zero),
        Peano255::Successor(PeanoEncoder::Successor(PeanoEncoder::Zero)),
    ] {
        println!("Machine representation of {:?}: {}", x, unsafe {
            std::mem::transmute::<_, u8>(x)
        })
    }
    let x = Peano1::Zero;
    println!("Machine representation of Peano1::{:?}: {}", x, unsafe {
        std::mem::transmute::<_, u8>(x)
    });
    for x in [
        Peano2::Successor(PeanoEncoder::Zero),
        Peano2::Zero,
    ] {
        println!("Machine representation of Peano2::{:?}: {}", x, unsafe {
            std::mem::transmute::<_, u8>(x)
        })
    }
    for i in 0_u8..=8 {
        let x = Peano255::transmute_u8(i);
        println!("transmute(u8::MAX - {}) = {:?}", i, x);
    }
    for i in 0_u8..=u8::MAX {
        let x = Peano255::transmute_u8(i);
        if i % 8 == 0 {
            print!("{:3} ", i)
        } else if i % 8 == 4 {
            print!(" ")
        }
        let c = if x.into_u8() == i { '✓' } else { '✗' };
        print!("{}", c);
        if i % 8 == 7 {
            println!()
        }
    }
    println!(
        "Size of Bin<Bin<Bin<Bin<()>>>>: {} byte",
        std::mem::size_of::<Bin<Bin<Bin<Bin<()>>>>>()
    );
}
