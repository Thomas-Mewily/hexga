use hexga_singleton::*;

pub mod prelude
{
    pub use super::*;
}

pub use hexga_random_core::*;

/*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rng;
impl Deref for Rng
{
    type Target=dyn RandomSource;
    fn deref(&self) -> &Self::Target { rng_ref().expect("Forget to init the rng") }
}
impl DerefMut for Rng
{
    fn deref_mut(&mut self) -> &mut Self::Target { rng_mut().expect("Forget to init the rng") }
}

impl Rng
{
    pub fn get() -> Option<&'static <Self as Deref>::Target> { rng_ref() }
    pub fn get_mut() -> Option<&'static mut<Self as Deref>::Target> { rng_mut() }
}
*/

//FIXME: Should deref on `&dyn RandomSource`, not `&Box<dyn RandomSource>`
singleton_thread_local!(pub Rng,Box<dyn RandomSource>,CONTEXT_RNG);


impl RandomSource for Rng
{
    fn next_u32(&mut self) -> u32 { self.as_mut().next_u32() }
    fn next_u64(&mut self) -> u64 { self.as_mut().next_u64() }
}


impl SingletonInit for Rng
{
    fn replace(value: Option<<Self as hexga_singleton::SingletonRef>::Target>) {
        CONTEXT_RNG.replace(value);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RandomSourceAllZero;




impl RandomSource for RandomSourceAllZero
{
    fn next_u64(&mut self) -> u64 { 0 }
    fn next_u32(&mut self) -> u32 { 0 }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RandomSourceAllOne;

impl RandomSource for RandomSourceAllOne
{
    fn next_u64(&mut self) -> u64 { u64::MAX }
    fn next_u32(&mut self) -> u32 { u32::MAX }
}


// Dummy temporary random source written by gpt... 
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct RandomSourceDummy {
    state: u64,
}

impl RandomSourceDummy {
    pub fn new(seed: u64) -> Self {
        Self { state : seed }
    }
}

impl RandomSource for RandomSourceDummy {
    fn next_u64(&mut self) -> u64 {
        // Standard xorshift64*
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        // Non-linear step: mix with a simple non-linear function (e.g., multiply by a prime and xor with rotated value)
        let non_linear = x.wrapping_mul(0x2545F4914F6CDD1D) ^ x.rotate_left(17);
        // Add a constant to avoid always-zero state, regardless of input
        const MIX_CONST: u64 = 0xA5A5A5A5A5A5A5A5;
        let result = non_linear.wrapping_add(MIX_CONST);
        self.state = result;
        result
    }
}