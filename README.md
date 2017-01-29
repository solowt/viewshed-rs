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

*  Write bilinear_interp function in utils.  Check API to see how this is done in terms of which 4 pixels are chosen.
*  Branch code and write a subset for finding viewshed + regions.  Drop image lib and much more in this branch.
*  Write methods to go between raster space and map space (web mercator to start with).  This uses constants attached to rasters, again: check getElevation() method.
*  Need to translate pathlist into lat/longs, x/ys, also consider the format we want to use.
*  Need to figure out how data gets transfered: if we compile to asm.js, what will path list look like? (array of arrays?)
*  Should we "poll" viewshed results?  IE, many pixels are tested more than once.  Currently last line to hit a pixel gets priority.  If we choose to poll, how do we store votes?  Hashmap?  Parallel arrays?
*  Find best tile in browser.  where is it?  Tilemap?  TileClass? Pool?  Check for elevation layer on change.  Search API.  Can also make another request if needed.  Where are requests being made from the API?  Note: check getElevation() in Tileworker (wrong name), this runs through children I believe.