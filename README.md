# viewshed-rs

## Goals

* Learn some rust.
* Learn some basic raster programming.
* Understand how viewsheds are calculated.
* Implement a simple viewshed algorithm.
* Generate some random elevation rasters and write code to generate viewshed rasters.
* Figure out how to move between raster space and map space.
* Move from a true-false raster to a multi-ring polygon (in web mercator or wgs84).
* Compile rust code to asm.js and/or wasm and run against in-browser elevation data.

## In-Browser Elevation Data

* Represented by f32 Array with known width.
* There should exist a simple method for moving between raster space and map space (find this in Esri JS API source code).
* All elevation data seems to be in 255X255 (array with ~65k elements) rasters.
* There is a root raster ("tile") and children tiles.
* Need to figure out relationship between parent and children tiles.

## TO DO (in order):

* Get PNG of elevation data
* Stop using arrays, only use vectors (array size must be known at compile time)
* Read in PNG, create Raster struct from PNG
* Perform viewshed on read-in PNG
* Add result_raster to PNG method (should be black and white)
