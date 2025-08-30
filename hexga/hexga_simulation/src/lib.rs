pub trait SimulationInput {}
pub trait SimulationResult {}

/* 
pub trait Simulation
{
    type Input : SimulationInput;
    type Result : SimulationResult;

    fn input(&mut self) -> Self::Input;
    fn update(&mut self, input: Self::Input) -> Option<Self::Result>;
    fn draw(&mut self) {}

    fn run(&mut self) -> Self::Result
    {

    }
}*/