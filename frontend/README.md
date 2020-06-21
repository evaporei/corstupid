# frontend 

It just consumes the `POST /user/info` route from the `backend` and renderers the JSON on the screen. It will use the `8080` port.

To compile/run use `wasm-pack build --target web --out-name wasm --out-dir ./static && miniserve ./static --index index.html`.

You may need to create a `static` folder with an `index.html` file with this content to actually run this project:

```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Yew Sample App</title>
        <script type="module">
            import init from "./wasm.js"
            init()
        </script>
    </head>
    <body></body>
</html>
```
