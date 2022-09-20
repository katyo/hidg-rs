use crate::Class;
use either::Either;

pub struct EitherReport<L, R> {
    inner: Either<L, R>,
}

impl<L, R> EitherReport<L, R> {
    pub fn new(inner: Either<L, R>) -> Self {
        Self { inner }
    }
}

deref_impl! {
    EitherReport<L, R> => inner: Either<L, R>,
}

impl<L, R> AsRef<[u8]> for EitherReport<L, R>
where
    L: AsRef<[u8]>,
    R: AsRef<[u8]>,
{
    fn as_ref(&self) -> &[u8] {
        match &self.inner {
            Either::Left(left) => left.as_ref(),
            Either::Right(right) => right.as_ref(),
        }
    }
}

impl<L, R> AsMut<[u8]> for EitherReport<L, R>
where
    L: AsMut<[u8]>,
    R: AsMut<[u8]>,
{
    fn as_mut(&mut self) -> &mut [u8] {
        match &mut self.inner {
            Either::Left(left) => left.as_mut(),
            Either::Right(right) => right.as_mut(),
        }
    }
}

impl<L: Class, R: Class> Class for Either<L, R> {
    type Input = EitherReport<L::Input, R::Input>;
    type Output = EitherReport<L::Output, R::Output>;

    fn input(&self) -> Self::Input {
        EitherReport::new(
            self.as_ref()
                .map_left(|class| class.input())
                .map_right(|class| class.input()),
        )
    }

    fn output(&self) -> Self::Output {
        EitherReport::new(
            self.as_ref()
                .map_left(|class| class.output())
                .map_right(|class| class.output()),
        )
    }
}
