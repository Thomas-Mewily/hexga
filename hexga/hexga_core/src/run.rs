
/// Run some code after some complexe initialization
pub trait Runner<F>
{
    type Output;
    type Param;
    fn run(f: F) -> Self::Output where Self::Param: Default
    {
        Self::run_with_param(f, Default::default())
    }
    fn run_with_param(f: F, param: Self::Param) -> Self::Output;
}

#[allow(async_fn_in_trait)]
pub trait AsyncRunner<F>
{
    type Output;
    type Param;
    async fn run(f: F) -> Self::Output where Self::Param: Default
    {
        Self::run_with_param(f, Default::default()).await
    }
    async fn run_with_param(f: F, param: Self::Param) -> Self::Output;
}
