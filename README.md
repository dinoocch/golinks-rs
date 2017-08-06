# golinks library

Simple service to manage url shortcuts.

Primarily intended for personal use on a LAN,
but *could* be external facing...

## Usage

aliases map to endpoints:
go/google -> https://google.com

GET requests to the alias return a redirect

OPTIONS -> returns definition for alias

## Setup

put it behind nginx :)

Add to your dns server within a search path.

Profit

## Implementation

hyper for HTTP

lmdb for storage ^.^

## Future work
