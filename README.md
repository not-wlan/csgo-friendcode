# CS:GO friend codes

This Rust library de- and encodes steam ids to and from CS:GO friend codes

## Related projects

While reverse engineering I've found the project [js-csfriendcode](https://github.com/emily33901/js-csfriendcode) by @emily33901.
It also provides the same functionality as this project but does not go as in depth with the reversing as this one.

## Decoding

CS:GO uses a simplified alphabet to encode friend codes, namely `"ABCDEFGHJKLMNPQRSTUVWXYZ23456789"`.
This alphabet has no ambiguous characters (think `1` and `I`).
Another useful side effect of this is that the alphabet size shrinks to 32 = 2^5.

To decode a friend code it is first extended to the full form by prepending `AAAA-`.
So `SUCVS-FADA` will become `AAAA-SUCVS-FADA`.
Once that's done the dashes will be removed, and our friend code will become `AAAASUCVSFADA`.
Each of these 13 characters is now mapped to its index in the custom alphabet. 
Since there's 32 possible values each character will be encoded with 5 bits and pushed onto a 64bit unsigned integer which subsequently is swapped in endianness.

If you're good at math in your head you will have noticed, that 13*5 = 65.
This is negligible since only the first padding byte will be affected by the truncation which is `A` which is `01000001` in binary so the first bit is 0 anyway.

The integer produced by this transformation will have the format `xxxxyxxxxyxxxxyxxxxyxxxxyxxxxyxxxxyxxxxy`, where the x represents the bits required for the steam id.
To extract them I've used the `pext` intrinsic function of the x86 architecture which gathers the bits specified by a mask into the contiguous low order bit positions of another integer.

Once that is done all that's left to do is to swap the nibbles in each byte, and a final endianness swap.
The resulting integer will be the account id of the associated steam account.
It can then be combined with the universe, account type and instance type to form the familiar 64bit steam id format.

## Encoding 

To encode a steam id into a CS:GO friend code the previous process is done in reverse.
The steam id will be scattered with the same mask used to gather the bits from the decoding step. 
This will however leave the bits that were filled by `y` in the previous step empty.

In theory every byte could be used to fill this void but CS:GO decided to use the first byte of the MD5 hash of the account id ORed with `b"CSGO\0\0\0\0"`.
The bits generated this way have their nibbles and endianness swapped in that order.

Both values are scattered using the opposite of `pext`, `pdep`. 
The resulting integers are combined with a bitwise OR and the result has its endianness swapped to form a final 64bit integer which consists of 13 5bit values which in turn each encode a letter of the friend code using the alphabet.
The dashes are then added by post processing as well as the removal of the `AAAA` prefix.

## Reversing 

The function to encode a steam id can be found by xrefing `"GetFriendCode"` in the client module of CS:GO and `"GetXuidFromFriendCode"` for the counterpart. 