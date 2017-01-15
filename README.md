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
* All elevation data seems to be in 256X256 (array with ~66049 (27^2) elements) rasters.
* There is a root raster ("tile") and children tiles.
* Need to figure out relationship between parent and children tiles.

## TO DO (in order):

*  Fix up .unwrap logic.  Move min/max to Raster.
*  Create CLI tool.
*  Prompt for data => read data.
*  Prompt for tasks: no data raster, print image, viewshed.
*  Viewshed => prompt for coords, save image
*  Back to prompt

*  Handle circle being drawn off raster.

*  Find best tile in browser.  where is it?  Tilemap?  Pool?  Check for elevation layer on change.  Search API.  Can also make another request if needed.