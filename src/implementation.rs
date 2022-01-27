use {
    crate::{IntoUrl, Response},
    reqwest::{self, Client},
    types::ApiResponseBody,
};

/// Gets the image url with the given client.
///
/// # Note
///
/// ## Context
///
/// this function returns [Future](std::future::Future) which must be awaited.
/// so you have to run this in an async context.
/// if you don't care about performance or only need blocking API,
/// check the [`blocking_get`](create::blocking_get) function out, which is blocking version of this.
///
/// ## Reusability
///
/// this function takes a [client](reqwest::Client) as an argument,
/// so if you have to make countless number of requests.\
/// it is good idea to reuse the same client for each request,
/// by using this function instead of [`blocking_get`].
///
/// most cases, however, will not need to call this directly.
/// if you find some simplest way, consider using [`blocking_get`](blocking_get) instead.
///
/// # Returns
///
/// On success, the [`Ok(String)`](String) which contains the `URL` of the image or GIF
/// found by using the given endpoint from the `API` returns.\
/// Otherwise, it will return the [`Err(NekosLifeError)`](NekosLifeError) if any error is encountered.
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// // declare the client we use
/// let client = reqwest::Client::new();
///
/// // and repeat the request 5 times.
/// for category in <
///     nekoslife::Category as strum::IntoEnumIterator
/// >::iter()
///     .take(5)
/// {
///     // get the url with blocking context. (no 'await')
///     // note that we pass the reference of the client
///     // as the first argument at here.
///     let url = nekoslife::get_with_client(
///         &client,
///         category,
///     ).await?;
///     // then print the each result.
///     println!("{url}");
/// }
/// # Ok::<(), nekoslife::NekosLifeError>(())
/// # });
/// ```
///
/// [get_with_client]: crate::get_with_client
pub async fn get_with_client<T>(
    client: &reqwest::Client,
    endpoint: T,
) -> crate::types::Result<<T as IntoUrl>::Response>
where
    T: IntoUrl,
{
    Ok(<T as IntoUrl>::parse(
        client
            .get(
                <T as IntoUrl>::into_url(endpoint)?
                    .to_string()
                    .as_str(),
            )
            .send()
            .await?,
    )
    .await?)
}

/// Gets the image url
///
/// # Note
///
/// ## Context
///
/// this function returns [Future](std::future::Future) which must be awaited.
/// so you have to run this in an async context.
/// if you don't care about performance or only need blocking API,
/// check the [`blocking_get`](create::blocking_get) function out, which is blocking version of this.
///
/// ## Reusability
///
/// also this function will make new [client](reqwest::Client)
/// struct with default settings every time it is called.\
/// if you have to reuse the client or set your client carefully,
/// consider using the [`get_with_client`] function instead.
///
/// # Returns
///
/// On success, the [`Ok(String)`](String) which contains the `URL` of the image or GIF
/// found by using the given endpoint from the `API` returns.\
/// Otherwise, it will return the [`Err(NekosLifeError)`](NekosLifeError) if any error is encountered.
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// // get the url from 'Waifu' category
/// let url = nekoslife::get(nekoslife::Category::Waifu).await?;
///
/// // then print the url.
/// println!("{url}");
/// # Ok::<(), nekoslife::NekosLifeError>(())
/// });
/// ```
///
/// [get_with_client]: crate::get_with_client
pub async fn get<T>(endpoint: T) -> crate::types::Result<<T as IntoUrl>::Response>
where
    T: IntoUrl,
{
    get_with_client(&Client::new(), endpoint).await
}

pub mod category;
pub mod types;

#[cfg(test)]
mod tests;

/// blocking version of implementation
#[cfg(feature = "blocking")]
pub mod blocking;
