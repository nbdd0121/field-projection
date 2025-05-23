use std::{marker::PhantomPinned, pin::Pin, task::Poll};

use field_projection::compat::{HasFields, p, start_proj};

#[derive(HasFields)]
#[fields(with_pinned)]
struct FairRaceFuture<F1, F2> {
    fair: bool,
    #[pin]
    f1: F1,
    #[pin]
    f2: F2,
    #[pin]
    _phantom: PhantomPinned,
}

impl<F1, F2> Future for FairRaceFuture<F1, F2>
where
    F1: Future,
    F2: Future<Output = F1::Output>,
{
    type Output = F1::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        start_proj!(mut self);
        let fair: &mut bool = p!(@mut self->fair);
        *fair = !*fair;
        match *p!(@self->fair) {
            true => {
                let f1: Pin<&mut F1> = p!(@mut self->f1);
                match f1.poll(cx) {
                    Poll::Pending => p!(@mut self->f2).poll(cx),
                    Poll::Ready(val) => Poll::Ready(val),
                }
            }
            false => match p!(@mut self->f2).poll(cx) {
                Poll::Pending => p!(@mut self->f1).poll(cx),
                Poll::Ready(val) => Poll::Ready(val),
            },
        }
    }
}

fn main() {}
