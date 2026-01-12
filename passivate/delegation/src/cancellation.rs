use std::error::Error;
use std::fmt::Display;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default, Clone)]
pub struct Cancellation
{
    flag: Arc<AtomicBool>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cancelled;

impl Cancellation
{
    pub fn is_cancelled(&self) -> bool
    {
        self.flag.load(Ordering::SeqCst)
    }

    pub fn cancel(&mut self)
    {
        self.flag.store(true, Ordering::SeqCst);
    }

    pub fn check(&self) -> Result<(), Cancelled>
    {
        if self.is_cancelled()
        {
            return Err(Cancelled);
        }

        Ok(())
    }
}

impl Error for Cancelled {}

impl Display for Cancelled
{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        Ok(())
    }
}
