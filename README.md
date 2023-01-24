# Hexagon Utility Library

This is a utility library written in Rust that eases development modules
that require working with the hexagon shape and related math.

This library is heavily based on
[Red Blob Games](https://www.redblobgames.com/grids/hexagons/implementation.html)
auto generated Rust implementation. Other implementations I looked at are messy and complicated.

The auto generated code actually was quite simple, but as such it was not implemented with Rust idiom in mind.

Edit: It is now much more idiomatic!
      Now, types use generic number types.

Packages are distributed under the
[zlib License](https://en.wikipedia.org/wiki/Zlib_License).

Feel free to improve and contribute using GitHub.
I would really like to see a tile-map manager at some point.

This library was developed in mind to be used as part of a
[BEVY](https://bevyengine.org/) game engine plugin.
However, BEVY implementation is not in the scope of this repository.
