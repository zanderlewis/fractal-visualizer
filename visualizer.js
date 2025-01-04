import init, { ViewState, FractalType, JuliaParams } from './pkg/fractal_visualizer.js';

async function run() {
    await init();

    const width = 1000;
    const height = 1000;
    const canvas = document.getElementById("canvas");
    canvas.width = width;
    canvas.height = height;
    const ctx = canvas.getContext("2d");

    let maxIterations = 50;
    let isDragging = false;
    let startX, startY;
    let needsRedraw = true;
    let fractalType = FractalType.Mandelbrot;

    let view = new ViewState(width, height, fractalType);

    function drawFractal() {
        if (!needsRedraw) return;
        needsRedraw = false;

        const escapeRadius = 4.0;
        const pixels = view.draw(maxIterations, escapeRadius);
        const imageData = new ImageData(new Uint8ClampedArray(pixels), width, height);
        ctx.putImageData(imageData, 0, 0);
    }

    canvas.addEventListener("wheel", (event) => {
        event.preventDefault();
        const factor = event.deltaY < 0 ? 0.9 : 1.1;  // Adjusted zoom factors
        view.zoom_at(event.offsetX, event.offsetY, factor);
        needsRedraw = true;
        requestAnimationFrame(drawFractal);
    });

    canvas.addEventListener("mousedown", (event) => {
        isDragging = true;
        startX = event.offsetX;
        startY = event.offsetY;
    });

    canvas.addEventListener("mousemove", (event) => {
        if (isDragging) {
            const dx = (event.offsetX - startX) / (width / 4.0);
            const dy = (event.offsetY - startY) / (height / 4.0);
            view.pan(-dx, -dy);
            startX = event.offsetX;
            startY = event.offsetY;
            needsRedraw = true;
            requestAnimationFrame(drawFractal);
        }
    });

    canvas.addEventListener("mouseup", () => {
        isDragging = false;
    });

    document.getElementById("iterations").addEventListener("change", (event) => {
        maxIterations = parseInt(event.target.value);
        needsRedraw = true;
        requestAnimationFrame(drawFractal);
    });

    document.getElementById("fractalType").addEventListener("change", (event) => {
        const selectedType = event.target.value;
        try {
            fractalType = FractalType[selectedType];
        } catch (e) {
            console.error(e);
            return;
        }
        view = new ViewState(width, height, fractalType);
        needsRedraw = true;
        requestAnimationFrame(drawFractal);
    });

    requestAnimationFrame(drawFractal);
}

run();