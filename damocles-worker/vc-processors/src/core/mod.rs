//! This module provides the most important types and abstractions
//!

use std::fmt::Debug;

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

pub mod ext;

/// Task of a specific stage, with Output & Error defined
pub trait Task
where
    Self: Serialize + DeserializeOwned + Debug + Send + Sync + 'static,
    Self::Output: Serialize + DeserializeOwned + Debug + Send + Sync + 'static,
{
    /// The stage name.
    const STAGE: &'static str;

    /// The output type
    type Output;
}

/// Processor of a specific task type
pub trait Processor<T: Task>
where
    Self: Send + Sync,
{
    /// Gets the Processor's name
    fn name(&self) -> String;
    /// Process the given task.
    fn process(&self, task: T) -> Result<<T as Task>::Output>;
}

impl<T: Task, P: Processor<T>> Processor<T> for Box<P> {
    fn name(&self) -> String {
        (**self).name()
    }

    fn process(&self, task: T) -> Result<<T as Task>::Output> {
        (**self).process(task)
    }
}

impl<T: Task> Processor<T> for Box<dyn Processor<T>> {
    fn name(&self) -> String {
        (**self).name()
    }

    fn process(&self, task: T) -> Result<<T as Task>::Output> {
        (**self).process(task)
    }
}

impl Task for () {
    const STAGE: &'static str = "EMPTY";
    type Output = ();
}

/// The DaemonProcessor does nothing
#[derive(Debug, Default, Clone, Copy)]
pub struct DaemonProcessor;

impl Processor<()> for DaemonProcessor {
    fn name(&self) -> String {
        String::new()
    }

    fn process(&self, _: ()) -> Result<<() as Task>::Output> {
        Ok(())
    }
}
