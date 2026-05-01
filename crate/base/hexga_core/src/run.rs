/// Run some code after some complexe initialization
pub trait Runner<F, P>
{
    type Output;
    fn run(f: F) -> Self::Output
    where
        P: Default,
    {
        Self::run_with_param(f, Default::default())
    }
    fn run_with_param(f: F, param: P) -> Self::Output;
}

#[allow(async_fn_in_trait)]
pub trait AsyncRunner<F, P>
{
    type Output;
    async fn run(f: F) -> Self::Output
    where
        P: Default,
    {
        Self::run_with_param(f, Default::default()).await
    }
    async fn run_with_param(f: F, param: P) -> Self::Output;
}
