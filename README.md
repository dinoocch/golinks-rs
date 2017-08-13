# golinks library

Simple service to manage url shortcuts.

## Disclaimer

This is primarily intended to be a quick introduction to rust rather than a
production ready application. It is unlikely to be developed further...

Therefore there are certainly instances of things that
could have been written better/more concisely. (namely with try! + functions)

LMDB is used on a whim.

## Usage

aliases map to endpoints:
go/google -> https://google.com

GET requests to the alias return a redirect

OPTIONS -> returns definition for alias

PUT -> http://go/test with data "https://google.com"
Replaces the alias test with the provided link.

Bonus points:
Use `{}` to accept arguments:
  * Say `go/google => https://google.com/search?q={}`
    Req `go/google/test` maps to `https://google.com/search?q=test`

As many arguments are allowed.  Only {} will be replaced.
Arguments that are not specified are left as {}...

## Setup

1. put it behind nginx :)
2. Add to your dns server within a search path / to `/etc/hosts`

## Implementation

hyper for HTTP

lmdb for storage
