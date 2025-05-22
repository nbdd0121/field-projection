use std::{pin::Pin, task::Poll};

use field_projection::compat::{HasFields, p, start_proj};

#[derive(HasFields)]
#[fields(with_pinned)]
struct FairRaceFuture<F1, F2> {
    fair: bool,
    #[pin]
    f1: F1,
    #[pin]
    f2: F2,
}

impl<F1, F2> Future for FairRaceFuture<F1, F2>
where
    F1: Future,
    F2: Future<Output = F1::Output>,
{
    type Output = F1::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut this = start_proj(self);
        let fair: &mut bool = p!(@mut this->fair);
        *fair = !*fair;
        match *p!(@this->fair) {
            true => {
                let f1: Pin<&mut F1> = p!(@mut this->f1);
                match f1.poll(cx) {
                    Poll::Pending => p!(@mut this->f2).poll(cx),
                    Poll::Ready(val) => Poll::Ready(val),
                }
            }
            false => match p!(@mut this->f2).poll(cx) {
                Poll::Pending => p!(@mut this->f1).poll(cx),
                Poll::Ready(val) => Poll::Ready(val),
            },
        }
    }
}

fn main() {}
