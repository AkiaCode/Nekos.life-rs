use {
    super::{
        get_with_client as async_get_with_client,
        types::IntoUrl,
    },
    reqwest::{self, Client},
};

/// Gets the image url in blocking context with the given client.
///
/// # Note
///
/// ## Context
///
/// this is the `blocking` version of the [`get_with_client`][get_with_client] function.\
/// if you want to use it in async contexts or performance is matters,
/// consider using the [`get_with_client`][get_with_client] function instead.
///
/// ## Reusability
///
/// this function takes a [client](reqwest::Client) as an argument,
/// so if you have to make countless number of requests.\
/// it is good idea to reuse the same client for each request,
/// by using this function instead of [`blocking::get`](get).
///
/// most cases, however, will not need to call this directly.
/// if you find some simplest way, consider using [`blocking::get`](get) instead.
///
#[doc = include_str!("../../docs/return.md")]
///
/// # Examples
///
/// ```rust,no_run
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
///     let url = nekoslife::blocking_get_with_client(
///         &client,
///         category,
///     )?;
///     // then print the each result.
///     println!("{url}");
/// }
/// # Ok::<(), nekoslife::NekosLifeError>(())
/// ```
///
/// [get_with_client]: crate::get_with_client
pub fn get_with_client<T>(
    client: &reqwest::Client,
    endpoint: T,
) -> crate::types::Result<<T as IntoUrl>::Response>
where
    T: IntoUrl,
{
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()?
        .block_on(async_get_with_client(&client, endpoint))
}

/// Gets the image url in blocking context.
///
/// # Note
///
/// ## Context
///
/// this is the `blocking` version of the [`get`][get] function.\
/// if you want to use it in async contexts or performance is matters,
/// consider using the [`get`][get] function instead.
///
/// ## Reusability
///
/// also this function will make new [client](reqwest::Client)
/// struct with default settings every time it is called.\
/// if you have to reuse the client or set your client carefully,
/// consider using the [`blocking::get_with_client`](get_with_client) function instead.
///
#[doc = include_str!("../../docs/return.md")]
///
/// # Examples
///
/// ```rust,no_run
/// let url: String = nekoslife::blocking_get(nekoslife::Category::Waifu)?;
/// # Ok::<(), nekoslife::NekosLifeError>(())
/// ```
///
/// [get]: crate::get
pub fn get<T>(
    endpoint: T,
) -> crate::types::Result<<T as IntoUrl>::Response>
where
    T: IntoUrl,
{
    get_with_client(&Client::new(), endpoint)
}

#[cfg(test)]
mod tests;
