## Context

this function returns [Future](std::future::Future) which must be awaited.
so you have to run this in an async context.

if you don't care about performance or only need blocking API,\
check the [`blocking`][blocking_version] function out,
which is blocking version of this.
