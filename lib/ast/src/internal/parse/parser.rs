// TODO: rename all of that to state machine once the old API is removed.

use std::fmt::Debug;

// TODO: parameterize on Input (a parser could handle different types of inputs, maybe?)
pub trait Ingest {
    type Input;
    type Ready;
    type Success;
    type Failure;

    fn ingest(self, input: Self::Input) -> IngestResult<Self::Ready, Self::Success, Self::Failure>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestResult<R, S, F> {
    Ready(R),
    Success(S),
    Failure(F),
}

impl<R, S, F> IngestResult<R, S, F>
where
    R: Debug,
    S: Debug,
    F: Debug,
{
    pub fn unwrap_ready(self) -> R {
        match self {
            IngestResult::Ready(ready) => ready,
            _ => panic!("cannot unwrap ready on {:?}", self),
        }
    }

    pub fn unwrap_success(self) -> S {
        match self {
            IngestResult::Success(success) => success,
            _ => panic!("cannot unwrap success on {:?}", self),
        }
    }

    pub fn unwrap_failure(self) -> F {
        match self {
            IngestResult::Failure(failure) => failure,
            _ => panic!("cannot unwrap failure on {:?}", self),
        }
    }
}

impl<R, S, F> IngestResult<R, S, F> {
    pub fn map_ready<NR>(self, function: impl FnOnce(R) -> NR) -> IngestResult<NR, S, F> {
        match self {
            IngestResult::Ready(ready) => IngestResult::Ready(function(ready)),
            IngestResult::Success(success) => IngestResult::Success(success),
            IngestResult::Failure(failure) => IngestResult::Failure(failure),
        }
    }

    pub fn map_success<NS>(self, function: impl FnOnce(S) -> NS) -> IngestResult<R, NS, F> {
        match self {
            IngestResult::Ready(ready) => IngestResult::Ready(ready),
            IngestResult::Success(success) => IngestResult::Success(function(success)),
            IngestResult::Failure(failure) => IngestResult::Failure(failure),
        }
    }

    pub fn map_failure<NF>(self, function: impl FnOnce(F) -> NF) -> IngestResult<R, S, NF> {
        match self {
            IngestResult::Ready(ready) => IngestResult::Ready(ready),
            IngestResult::Success(success) => IngestResult::Success(success),
            IngestResult::Failure(failure) => IngestResult::Failure(function(failure)),
        }
    }

    pub fn on_failure<NF>(
        self,
        function: impl FnOnce(F) -> IngestResult<R, S, NF>,
    ) -> IngestResult<R, S, NF> {
        match self {
            IngestResult::Ready(ready) => IngestResult::Ready(ready),
            IngestResult::Success(success) => IngestResult::Success(success),
            IngestResult::Failure(failure) => function(failure),
        }
    }
}

pub trait Finalize
where
    Self: Sized,
{
    type Result;

    fn finalize(self) -> Self::Result;
}

pub trait Parser
where
    Self: Ingest + Finalize,
{
}

impl<T> Parser for T where T: Ingest + Finalize {}
