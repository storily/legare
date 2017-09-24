# Legare

Parsing services for Cogitare.

## Normalisation

- Smart quotes are converted to straight quotes
- Dashes are converted to hyphens
- Whitespace is converted to spaces
- Consecuting whitespace is collapsed
- Brackets are converted to parens
- `:` is converted to `=`
- Everything is lowercased

## Parsing

A very simple syntax:

- Words (any sequence not matching others)
- IDs (`#123`) → parsed to uints
- Quotes (`"foo bar"`) → not parsed further
- Logic keywords (`and`, `or`, `not`)
- Pairs (`abc=def`)
- Groups (`(ash tg=y "neoi")`) and subgroups

## Server

A simple server that takes the request's body (responding to any method),
normalises it, parses it, and returns the AST as JSON. In the debug builds,
the JSON is pretty-printed.

While the HTTP status should be considered, for convenience all responses
include an `"error"` field, which contains either the error category
(currently, `io` or `parse`), or `false`.

In the case of a `parse` error, a rich human-friendly multiline error display
is provided. While that shouldn't be used for user-facing interfaces, it can be
very useful when debugging.

## Setup

Will either bind to the env's `PORT` or listen on a `LISTEN_FD` provided.
