use std::{cell::RefCell, sync::{Mutex, OnceLock}};

static LOGS: Mutex<RefCell<Vec<String>>> = Mutex::new(RefCell::new(Vec::new()));

static SPY_LOGGER: OnceLock<SpyLogImpl> = OnceLock::new();

#[derive(Debug)]
struct SpyLogImpl;

pub struct SpyLog;

impl SpyLog
{
    pub fn set() -> SpyLog
    {
        SPY_LOGGER.set(SpyLogImpl).expect("failed to set spy log (1)");

        let spy: &'static SpyLogImpl = SPY_LOGGER.get().unwrap();
        
        log::set_logger(spy).map(|()|
        {
            log::set_max_level(log::LevelFilter::Info);    
        }).expect("failed to set spy log (2)");

        SpyLog
    }
}

impl log::Log for SpyLogImpl
{
    fn enabled(&self, _metadata: &log::Metadata) -> bool
    {
        true
    }

    fn log(&self, record: &log::Record)
    {
        let logs = LOGS.lock().expect("failed to lock spy log");

        logs.borrow_mut().push(format!("{} - {}", record.level(), record.args()));
    }

    fn flush(&self) {}
}

impl IntoIterator for SpyLog
{
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;


    fn into_iter(self) -> Self::IntoIter 
    {
        let logs = LOGS.lock().expect("failed to lock spy log");

        logs.borrow().clone().into_iter()
    }
}
