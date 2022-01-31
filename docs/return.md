# Returns

On success, the [`Ok(<T as IntoUrl>::Response)`][response] will return,\
which contains the response found by using the given endpoint at the API.\
Otherwise, it will return the [`Err(Error)`](crate::Error) if any error was encountered.

The `Response` type varies, and they depend on
the implementations of [`IntoUrl`](crate::IntoUrl).

For example, if you passed [`Category`](crate::Category),
you may receive [`UrlString`](crate::UrlString)
(the URL of the image or the GIF) which is just an alias for [String].\
On the other hand, if you passed [`EightBall`](crate::EightBall),
the `Response` will be [`EightBallResponse`](crate::EightBallResponse) type.

And actually, all of the return types are [`Future`](std::future::Future),
so you may have to `.await` it to get the result.

[response]: crate::IntoUrl#associatedtype.Response
