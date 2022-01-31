## Reusability

also this function will make new [client](reqwest::Client)
struct with default settings every time it is called.

if you have to reuse the client or set your client carefully,\
consider using the [`get_with_client`][get_with_client] function instead.
