## Magic Bitboards

This document is a brief self note to jot down my understanding of Magic Bitboards for future reference, you can ignore this document if you already understand Magic Bitboards.

The motivation behind writing this is that the wiki page for magic bitboards was poorly written and I didn't find a great reference explaining it properly and completely. I spent quite a bit of time trying to understand how it works and didn't want to have to try to relearn it all over again.

Disclaimer : This document is pretty basic, and it's a very basic understanding of bitboards, there may be other techniques which I haven't read about. If anything I mentioned in this document does not make sense / is not correct - do let me know.

### Why do magic bitboards exist?
Calculating the lookups for pieces that just jump squares - like kings, pawns, knights - are straightforward. From every square there is exactly one set of squares it can reach / attack. (In the case of pawns, black and white would have different results because they go in different directions, but knights and kings have same attacks irrespective of color).

In the case of sliding pieces like rooks and bishops on the other hand, from each square there are a lot of possibilities of whereall it can 'reach' depending on the current state of the board. (i.e. what all pieces are placed where - this is the case because some arbitrary piece might be blocking your piece from going beyond a certain square).
Taking a simple example, if there was a Bishop on e5 and there was a random piece on d4, the valid attack squares would be - d4, f6, g7, h8, f4, g2, h1, d6, c7, b8
Here d4 sort of 'prunes' off the rest of the possible attacking squares - c3, b2, a1

For the above example in particular, in the best case, there will be no blocking pieces and hence we would have 13 squares to attack.
One small optimization that can be added here is that the edge squares don't really matter for this calculation. That is, whether or not there was a piece there, you can technically 'attack' that square (regardless of what color the piece it is - you dont have to necessarily capture the piece, so even a same squared piece is fine).
To make it a bit easier to understand - the same example above has an edge square b8. If there was no piece on c7 or d6, we would definitely be able to attack ANY piece on b8. In case there WAS a piece on d6, we know that the rest of the branch will be 'pruned'.
In both cases, we don't care what piece is on b8. Keep this small point in mind for a few more lines down.

Now, we can simply run a for loop from a given square in the 4 directions, caring for any blocking pieces, and find the valid attacks.
This is correct. The purpose of magic bitboards is NOT to assist in calculating these attacks, but rather to store / create a lookup table for / memoize for this calculation we are doing.
As you probably thought of by now, there doesn't seem to be any simple way to reference "I am on this square and these are the blocking pieces in my horizontal and vertical (for rooks) / diagonal (for bishops) line of sight" to be able to create an index for all combinations.
This is where magic numbers, and magic bitboards come in. We want to be able to - given a board position, and a square - map a certain scenario to a number to be able to index it in a lookup table.

So we know for sure that we need to calculate attacks for each position on the board, and for each blocker combination possibility. The first half is easy, we have 64 squares so we can know that we have a starting point atleast.
Let's take bishop attacks in particular for this. The lookup table would look something like.
```
ulong [,] bishopTable = new [64, max_index_value]
```
Now going back to the example two paragraphs above, we know that there are 13 possible squares on which blocking pieces can exist. So max_index_value is basically `1 << 13` at most (one bit for whether a blocking piece is present each position  - how we map these positions we will get to).
If you remember the optimization, we can avoid the edge pieces and hence there are atmost `1 << 9` indexes.

Now that we know the number of possibilities, how do we map each combination of blocking pieces?
If we have a position of blocking pieces being (same e5 example - we don't care about pieces not on the same diagonal)
```
0 0 0 0 0 0 0 0
0 0 1 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 X 0 0 0
0 0 0 0 0 1 0 0
0 0 1 0 0 0 0 0
0 0 0 0 0 0 0 1
1 0 0 0 0 0 0 0
```
Say we want to represent it as `100001100` (going left -> right and top -> bottom and only considering the bits on the relevant diagonal and also excluding edge bits)

Let's call the first number (the big binary matrix above) as X and the second representation as P.
A magic number is basically a unique number for a square such that X * Magic Number = P, for all one-to-one mappings for X -> P
I didn't look too much into the mathematical proof for if and why such a number could exist, will probably expand this document if I go to look into that.
But with this understanding of what magic numbers are and how they work - you would (hopefully) be able to better understand the wiki and the algorithms mentioned there.