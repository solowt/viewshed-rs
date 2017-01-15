var http = require('http');
var Lerc = require('lerc');
var prompt = require('prompt'); 
var fs = require('fs');

prompt.start();

// while(true){
  prompt.get(['url'], function (err, result) {
    getData(result.url);

  });
// }

function getData(url){
  http.get(url, function (res) {
    var data = [];
    res.on('data', function (chunk) {
      // append each chunk to a list of buffer objects 
      data.push(chunk);
    })
    res.on('end', function () {
      // turn the list into one large Buffer 
      data = Buffer.concat(data);
      // because the decoder expects an ArrayBuffer 
      var image = Lerc.decode(data.buffer);
      console.log("width of output is: " + image.width);
      var arr = [];
      // console.log(image)
      // console.log(image.pixels.length)
      for (var k in image.pixels[0]){
        arr.push(image.pixels[0][k])
      }

      fs.writeFile('test.txt', JSON.stringify(arr), function(err) {
        if(err) {
          return console.log(err);
        }
        console.log("The file was saved!");
      });

    })
  });
} 

 