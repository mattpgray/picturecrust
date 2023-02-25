# PicturecRUST

PicturecRUST is a terrible picture cross solver, written in rust.

It works by recursively generating all the rows and columns based on the column and row settings,
and then comparing them so see if they match.

If a board generated from row setting matches a board generated from col settings, we have found
the winning board.

The current implementation is very slow, and should only be used on 8x8 boards.

