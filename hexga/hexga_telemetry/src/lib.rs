use hexga_utils::scope::ScopedWith;


pub struct Telemetry;

impl ScopedWith<&'static str> for Telemetry
{
    fn begin(&mut self, value : &str) 
    {
        let _ = value;
    }

    fn end(&mut self) {
        
    }
}