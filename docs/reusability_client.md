## Reusability

this function takes a [client](reqwest::Client) as an argument.

so if you have to make countless number of requests,\
it is good idea to reuse the same client for each request,
by using this function instead of [`get`][get].

most cases, however, will not need to call this directly.\
if you find some simplest way, consider using [`get`][get] instead.
