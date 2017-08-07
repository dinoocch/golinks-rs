# golinks library

Simple service to manage url shortcuts.

Primarily intended for personal use on a LAN,
but *could* be external facing...

Original concept credit to google.  There's lots of other adaptations out there.

The concept is far too easy to get used to...plus I like these better than
bookmarks.

## Disclaimer

This is primarily intended to be a quick introduction to the
rust programming language and an easy Sunday project.

Therefore there are certainly instances of things that
could have been written better/more concisely.

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

Future may make arguments optional ()

## Setup

put it behind nginx :)

Add to your dns server within a search path.

Profit

## Implementation

hyper for HTTP

lmdb for storage ^.^

## What is not there (yet?)

* Authentication (PUT/DELETE)
* Web interface
* High Availability/Scalability

These features will likely never exist.
