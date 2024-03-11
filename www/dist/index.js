import { render, renderer } from "svgbob-wasm";

var source = document.getElementById("svgbob-source").innerHTML;

let dest = document.getElementById("svgbob-dest");
dest.innerHTML = render(source);

var custom_dest = document.getElementById("svgbob-custom-dest");
custom_dest.innerHTML = renderer()
    .background("black")
    .stroke_color("white")
    .fill_color("white")
    .render(source);
