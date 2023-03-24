import init, {BilateralFilter} from '../pkg/bilateral_filter.js';

let originalCvs;
let filteredCvs;

let originalCtx;
let filteredCtx;
let filteredImageData;

let grayScaleArray;

let sigmaSpatial;
let sigmaIntensity;

let bilateralFilter;

let imageSize = 0;

let memory;

const inputImage = new Uint8Array(36);
inputImage.set([
    205, 185, 193, 105, 135, 93,
    205, 189, 193, 115, 116, 13,
    215, 142, 124, 125, 181, 73,
    108, 185, 161, 135, 135, 83,
    65, 185, 53, 119, 135, 93,
    89, 185, 193, 105, 135, 93,
]);

window.addEventListener('DOMContentLoaded', async (event) => {
    const output = await init();
    memory = output.memory;

    sigmaSpatial = +document.getElementById('sigmaSpatial').value;
    sigmaIntensity = +document.getElementById('sigmaIntensity').value;

    const image = document.createElement('img');

    image.addEventListener("load", (e) => {
        const width = image.width.toString();
        const height = image.height.toString();

        imageSize = width * height;

        originalCvs = document.getElementById("originalCvs");

        originalCvs.setAttribute('width', width);
        originalCvs.setAttribute('height', height);
        originalCvs.style.width = width;
        originalCvs.style.height = height;

        originalCtx = originalCvs.getContext("2d");

        originalCtx.drawImage(image, 0, 0);

        filteredCvs = document.getElementById("filteredCvs");

        filteredCvs.setAttribute('width', width);
        filteredCvs.setAttribute('height', height);
        filteredCvs.style.width = width;
        filteredCvs.style.height = height;

        filteredCtx = filteredCvs.getContext("2d");

        filteredImageData = filteredCtx.getImageData(0, 0, +width, +height);
        
        // Canvas have rgba chanels but i need only one since my image doesn't has colors
        grayScaleArray = getGrayscaleArray(originalCtx);
        bilateralFilter = BilateralFilter.new(width, height);
        bilateralFilter.set_sigma(sigmaSpatial, sigmaIntensity);
        new Uint8Array(memory.buffer, bilateralFilter.input_data_ptr(), imageSize).set(grayScaleArray);

        applyBilateralFilter();
    });
    
    const selectTestImage = document.getElementById('testImages');
    selectTestImage.onchange = e => {
        const imageName = e.currentTarget.value;

        if(imageName.length > 0) {
            image.src = `./images/${imageName}`;
        }
    }
});

function getGrayscaleArray(ctx) {
    const imageData = ctx.getImageData(0, 0, ctx.canvas.width, ctx.canvas.height);
    const data = imageData.data;
    const grayScaleArray = new Uint8Array(data.byteLength / 4);

    // Took only "r" chanel
    for (let i = 0, j = 0; i < data.length; i+=4, j++) {
        grayScaleArray[j] = data[i];
    }

    return grayScaleArray;
}

function fillCanvasData(filteredArray, ctx) {
    const data = new Uint32Array(filteredImageData.data.buffer);

    // Fill all 4 chanles with only one operation on abgr way
    for (let i = 0; i < filteredArray.length; i++) {
        data[i] = (255 << 24) + (filteredArray[i] << 16) + (filteredArray[i] << 8) + filteredArray[i];
    }

    ctx.putImageData(filteredImageData, 0, 0);
}

async function applyBilateralFilter() {
    const t0 = performance.now();
    bilateralFilter.run();
    console.log(performance.now() - t0);

    const outputPtr = bilateralFilter.output_data_ptr();
    const outputImage = new Uint8Array(memory.buffer, outputPtr, imageSize);
    
    fillCanvasData(outputImage, filteredCtx);
}

document.getElementById('sigmaSpatial').onchange = e => {
    sigmaSpatial = +e.target.value;
    document.getElementById('sigmaSpatialText').textContent = sigmaSpatial.toString();
    bilateralFilter.set_sigma(sigmaSpatial, sigmaIntensity);
    applyBilateralFilter();
}
document.getElementById('sigmaIntensity').onchange = e => {
    sigmaIntensity = +e.target.value;
    document.getElementById('sigmaIntensityText').textContent = sigmaIntensity.toString();
    bilateralFilter.set_sigma(sigmaSpatial, sigmaIntensity);
    applyBilateralFilter();
}