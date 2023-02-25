# PicturecRUST

PicturecRUST is a terrible picture cross solver, written in rust.

It works by recursively generating all the rows and columns based on the column and row settings,
and then comparing them so see if they match.

If a board generated from row setting matches a board generated from col settings, we have found
the winning board.

The current implementation is very slow, and should only be used on 8x8 boards.

## TODO

- Add command line options to avoid having to specifiy the board in code directly.
  (The coolest thing would be to take a picture of a board and load the settings from that.
   I doubt that will ever happen in this project though. Maybe just loading from json.)
- Speed up the solving time. Sould work on 15x15 boards within a reasonable time frame.

