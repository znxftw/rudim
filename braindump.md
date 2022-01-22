- Piece conversion logic seems to be growing a bit disconnected
- Assertions for FENTests could possibly be improved, seems unnecessarily long? Loop over an array of expected values in the same order?
- Improve the tests, maybe parameterised tests, for better coverage? Are all these unit tests doing anything - it still doesn't ensure every single of the 64 possibilities work, why is it checking just the corner squares?
- Have linting in place soon, lots of small code smells are creeping in slowly.
- Test out bishop, rook tables manually

- Currently implementing : Move generation


Perft results seem unbelievably slow - need to fix the move generation speed (or maybe a problem in my test run?) before going to implementing UCI