# Hexagon Utility Library

This is a utility library written in Rust that eases development modules
that require working with the hexagon shape and related math.

This library is heavily based on
[Red Blob Games](https://www.redblobgames.com/grids/hexagons/implementation.html)
auto generated Rust implementation. Other implementations I looked at are messy and complicated.
The auto generated code actually was quite simple,
but as such it was not implemented with Rust idiom in mind.

**This implementation is different in few ways:**

* Both `Hex` and `FractionalHex` implement `PartialEq`.  
  This means you can do `assert_eq!(hex1, hex2)` or `hex1 == hex2`.
* `Point` also have `PartialEq` implementation for `f64` using `float_eq`. 
* I implemented some of the function as traits of `Hex` and `FractionalHex`
  and this means you can do `hex1.add(hex2)` or `hex1.round()`.
* Other functionality is implemented as static functions
  in their own objects `HexDirection`, `HexOffset`, `HexDoubled`.
* Tests cover everything, and implemented in Rust way.
* The auto-generated code split into several file modules.

This packages is distributed under the
[zlib License](https://en.wikipedia.org/wiki/Zlib_License).

Feel free to improve and contribute using github.
I would really like to see a tile-map manager at some point.

This library was developed in mind to be used as part of a
[BEVY](https://bevyengine.org/) game engine plugin.
However BEVY implementation is not in the scope of this repository.
