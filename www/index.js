import { convert_string } from "svgbob-wasm";

var source = document.getElementById("svgbob-source").innerHTML;
var dest = document.getElementById("svgbob-dest");
dest.innerHTML = convert_string(source);
