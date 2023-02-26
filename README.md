# PicturecRUST

PicturecRUST is a picture cross solver, written in rust.

It uses the following procedure to solve a picture cross puzzle:

1: Generate a list of proposals for each row and column based on the row and column settings.
   For example, for a 8x8 board with a row setting of (3, 3) has 3 possible configurations:

```
1. ███X███X
2. ███XX███
3. X███X███
```
2. For the row proposals, generate a board where the state of each cell is known by filling
   in the cells that overlap. Wfter generating this board, filter out any col proposals that do
   not match the board. For example, for the proposals above, the generated row would be:

```
?██??██?
?██??██?
?██??██?
```

3. Do the same thing but with the row proposals swapped, using the filtered col proposals from step 2

4. Check how many possible boards are left by multiplying all the number of proposals. This will
   be two different values for each of the row and col proposals. If there are 0 valid boards,
   the settings are invalid and we can stop. If there are exactly 1 valid row and col board, we have
   found our board and we are done. If there number of valid boards has not changed since the last
   stage, we are stuck and we start brute forcing in step 5.

5. Generate all possible board given the row and col proposals. Check if there is a board that
   exists in both the sets of board. If there is, we have found the winning board. If there is not
   the settings are incorrect.

## TODO

- Add command line options to avoid having to specifiy the board in code directly.
  (The coolest thing would be to take a picture of a board and load the settings from that.
   I doubt that will ever happen in this project though. Maybe just loading from json.)

