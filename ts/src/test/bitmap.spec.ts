import { Canvas, CanvasRenderingContext2D, createCanvas, createImageData, Image, ImageData } from "canvas";
import * as chai from "chai";
import * as fs from "fs";
import { JsonReader } from "kryo/readers/json";
import sysPath from "path";
import { $DefineBitmap, DefineBitmap } from "swf-types/tags";
import { fileURLToPath, pathToFileURL, URL } from "url";
import { decodeXSwfBmpSync } from "../lib/decode/x-swf-bmp";
import { SwfBitmap } from "../lib/image";
import { compareImages, ImageComparison } from "./compare-pixels";
import meta from "./meta.js";

const PROJECT_ROOT: string = sysPath.join(meta.dirname, "..", "..", "..");
const TEST_SAMPLES_ROOT: string = sysPath.join(PROJECT_ROOT, "..", "tests", "bitmap", "swfll1");
const JSON_READER: JsonReader = new JsonReader();

describe("bitmap", function () {
  it("x-swf-bmp", async function () {
    const inputJson: string = fs.readFileSync(
      sysPath.join(TEST_SAMPLES_ROOT, "homestuck-beta-3", "tag.json"),
      {encoding: "UTF-8"},
    );
    const bitmapTag: DefineBitmap = $DefineBitmap.read(JSON_READER, inputJson);
    const actualBitmap: SwfBitmap = decodeXSwfBmpSync(bitmapTag.data);

    const actualPng: Uint8Array = swfBitmapToPngSync(actualBitmap);
    fs.writeFileSync(sysPath.join(TEST_SAMPLES_ROOT, "homestuck-beta-3", "local-actual.ts.png"), actualPng);

    const expectedImage: Image = await readPng(pathToFileURL(sysPath.join(TEST_SAMPLES_ROOT, "homestuck-beta-3", "expected.png")));
    const expected: ImageData = asImageData(expectedImage);

    const diff: ImageComparison = compareImages(swfBitmapToImageData(actualBitmap), expected, {warn: 0, error: 0});

    if (diff.error > 0) {
      const diffPng: Uint8Array = imageDataToPngSync(diff.diff);
      fs.writeFileSync(sysPath.join(TEST_SAMPLES_ROOT, "homestuck-beta-3", "local-diff.ts.png"), diffPng);
    }
    chai.assert.strictEqual(diff.error, 0);
  });
});

function swfBitmapToImageData(swfBitmap: SwfBitmap): ImageData {
  return createImageData(swfBitmap.data, swfBitmap.width, swfBitmap.height);
}

function swfBitmapToPngSync(swfBitmap: SwfBitmap): Uint8Array {
  const imageData: ImageData = createImageData(swfBitmap.data, swfBitmap.width, swfBitmap.height);
  return imageDataToPngSync(imageData);
}

function imageDataToPngSync(imageData: ImageData): Uint8Array {
  const canvas: Canvas = createCanvas(imageData.width, imageData.height);
  canvas.getContext("2d").putImageData(imageData, 0, 0);
  return canvas.toBuffer("image/png");
}

type ImageLike = Canvas | Image;

function asImageData(input: ImageLike): ImageData {
  const canvas: Canvas = createCanvas(input.width, input.height);
  const cx: CanvasRenderingContext2D = canvas.getContext("2d");
  cx.drawImage(input, 0, 0);
  return cx.getImageData(0, 0, input.width, input.height);
}

async function readPng(path: URL): Promise<Image> {
  return new Promise((resolve, reject) => {
    const img: Image = new Image();
    img.onerror = onError;
    img.onload = onLoad;
    img.src = fileURLToPath(path);
    return teardown;

    function onLoad() {
      teardown();
      resolve(img);
    }

    function onError(err: Error): void {
      teardown();
      reject(err);
    }

    function teardown() {
      delete img.onerror;
      delete img.onload;
    }
  });
}
