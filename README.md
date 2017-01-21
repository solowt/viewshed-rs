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

*  Review get_height_recurr in raster.rs - do we need it?  Is it causing issues?  Instead just take the true/false from last pixel calculated.
*  Is circle being drawn off raster?  Are lines being drawn off raster?  Fix if so.
*  Should we "poll" viewshed results?  IE, many pixels are tested more than once.  Currently last line to hit a pixel gets priority.  If we choose to poll, how do we store votes?  Hashmap?  Parallel arrays?
*  Find best tile in browser.  where is it?  Tilemap?  TileClass? Pool?  Check for elevation layer on change.  Search API.  Can also make another request if needed.  Where are requests being made from the API?  Note: check getElevation() in Tileworker (wrong name), this runs through children I believe.
*  Write methods to go between raster space and map space (web mercator to start with).  This uses constants attached to rasters, again: check getElevation() method.
*  Write method to find polygons in a result raster.  Result should be Type: Vec<Vec<Point>>. Need to find the "edges" of a polygon -> true pixels bordering a false pixel on any of the 8 bordering pixels.
*  Add slope raster prop to raster, holds the slope raster.
*  Write method to find all slopes under a certain distance from 0 in slope pixels.  These regions also need to be turned into polygons, simlar to viewshed result rasters.