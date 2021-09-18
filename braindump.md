- Piece conversion logic seems to be growing a bit disconnected
- Assertions for FENTests could possibly be improved, seems unnecessarily long? Loop over an array of expected values in the same order?
- Improve the tests, maybe parameterised tests, for better coverage? Are all these unit tests doing anything - it still doesn't ensure every single of the 64 possibilities work, why is it checking just the corner squares?
- Test out bishop, rook tables manually

- Currently implementing : Move generation