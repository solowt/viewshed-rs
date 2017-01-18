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

## TO DO:

*  Separate code into modules
*  Make circle struct with rad, center, pixels
*  Handle circle being drawn off raster.
*  Should we "poll" viewshed results?  IE, some pixels are tested more than once.
*  Find best tile in browser.  where is it?  Tilemap?  TileClass? Pool?  Check for elevation layer on change.  Search API.  Can also make another request if needed.  Where are requests being made from the API?
*  Write methods to go between raster space and map space (web mercator to start with).
*  Write method to find polygons in a result raster.  Result should be Type: Vec<Vec<Point>>. Need to find the "edges" of a polygon -> true pixels bordering a false pixel on any of the 8 bordering pixels.  Easy to find these, need to find a way to order them into series of point to make a polygon.