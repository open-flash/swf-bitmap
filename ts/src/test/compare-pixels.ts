import { createImageData, ImageData } from "canvas";

export interface CompareOptions {
  warn: number;
  error: number;
}

export interface ImageComparison {
  diff: ImageData;
  sameSize: boolean;
  error: number;
  relativeError: number;
}

interface StraightSrgba {
  r: number;
  g: number;
  b: number;
  a: number;
}

const OK_COLOR: StraightSrgba = {r: 255, g: 255, b: 255, a: 255};
const WARN_COLOR: StraightSrgba = {r: 255, g: 255, b: 0, a: 255};
const ERR_COLOR: StraightSrgba = {r: 255, g: 0, b: 0, a: 255};
const OOB_COLOR: StraightSrgba = {r: 0, g: 0, b: 0, a: 255};

export function compareImages(actual: ImageData, expected: ImageData, options: CompareOptions): ImageComparison {
  const diffWidth: number = Math.max(actual.width, expected.width);
  const diffHeight: number = Math.max(actual.height, expected.height);
  const sameSize: boolean = actual.width === expected.width && actual.height === expected.height;
  const diff: ImageData = createImageData(diffWidth, diffHeight);
  let error: number = 0;
  for (let y: number = 0; y < diffHeight; y++) {
    for (let x: number = 0; x < diffWidth; x++) {
      const actualSrgb: StraightSrgba | undefined = getPixel(actual, x, y);
      const expectedSrgb: StraightSrgba | undefined = getPixel(actual, x, y);
      let diffSrgb: StraightSrgba;
      if (actualSrgb === undefined || expectedSrgb === undefined) {
        diffSrgb = OOB_COLOR;
        error += 255 * 4;
      } else {
        const err: number = comparePixel(actualSrgb, expectedSrgb);
        error += err;
        if (err > options.error) {
          diffSrgb = ERR_COLOR;
        } else if (err > options.warn) {
          diffSrgb = WARN_COLOR;
        } else {
          diffSrgb = OK_COLOR;
        }
      }
      setPixel(diff, x, y, diffSrgb);
    }
  }
  const relativeError: number = error / (diffWidth * diffHeight * 255 * 4);
  return {diff, sameSize, error, relativeError};
}

function comparePixel(actual: StraightSrgba, expected: StraightSrgba): number {
  return Math.abs(actual.r - expected.r)
    + Math.abs(actual.g - expected.g)
    + Math.abs(actual.b - expected.b)
    + Math.abs(actual.a - expected.a);
}

function getPixel(data: ImageData, x: number, y: number): StraightSrgba | undefined {
  if (x < data.width && y < data.height) {
    const offset: number = (y * data.width + x) * 4;
    return {
      r: data.data[offset],
      g: data.data[offset + 1],
      b: data.data[offset + 2],
      a: data.data[offset + 3],
    };
  } else {
    return undefined;
  }
}

function setPixel(data: ImageData, x: number, y: number, color: StraightSrgba): void {
  if (x < data.width && y < data.height) {
    const offset: number = (y * data.width + x) * 4;
    data.data[offset] = color.r;
    data.data[offset + 1] = color.g;
    data.data[offset + 2] = color.b;
    data.data[offset + 3] = color.a;
  }
}
