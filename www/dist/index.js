import { render } from "svgbob-wasm";

var source = document.getElementById("svgbob-source").innerHTML;
var dest = document.getElementById("svgbob-dest");
dest.innerHTML = render(source);
