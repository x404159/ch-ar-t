# Convert jpg image to ascii

## usage

````bash
# with url
cargo run --release -- --url https://something.jpg

# with url and different texture (optional)
cargo run --release -- --url https://something.jpg -t 2

# with url and different texture (optional), resize width (optional, defaults to terminal width)
cargo run --release -- --url https://something.jpg -t 2 --width 100

# with path
cargo run --release -- --path something.jpg
````

## Result

### input image

![input image](./assets/sample.jpg)

#### output

![output ascii](./assets/result_sample.png)

