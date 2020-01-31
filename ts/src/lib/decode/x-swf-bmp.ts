import { ReadableByteStream, ReadableStream } from "@open-flash/stream";
import { inflate } from "pako";
import { Uint32, Uint8, UintSize } from "semantic-types";
import { SwfBitmap } from "../image";

const RGB_SIZE: UintSize = 3;
const RGBA_SIZE: UintSize = 4;

export function decodeXSwfBmpSync(bytes: Uint8Array): SwfBitmap {
  const stream: ReadableByteStream = new ReadableStream(bytes);
  const formatCode: Uint8 = stream.readUint8();
  if (formatCode !== 3) {
    throw new Error(`UnsupportedXSwfBmpFormatCode: ${formatCode}`);
  }
  const width: UintSize = stream.readUint16LE();
  const height: UintSize = stream.readUint16LE();
  const srcStride: UintSize = width + ((4 - (width % 4)) % 4);
  const colorCount: UintSize = stream.readUint8() + 1;
  const colors: Uint32[] = [];
  const compressedData: Uint8Array = stream.tailBytes();
  const srcData: Uint8Array = inflate(compressedData);
  const data: Uint8ClampedArray = new Uint8ClampedArray(width * height * RGBA_SIZE);
  const dataView: DataView = new DataView(data.buffer, data.byteOffset, data.byteLength);
  const colorTableSize: UintSize = RGB_SIZE * colorCount;
  for (let i: UintSize = 0; i < colorTableSize; i += 3) {
    const r: Uint8 = srcData[i];
    const g: Uint8 = srcData[i + 1];
    const b: Uint8 = srcData[i + 2];
    colors.push((r * 2 ** 24) + (g << 16) + (b << 8) + 0xff);
  }
  for (let y: UintSize = 0; y < height; y++) {
    for (let x: UintSize = 0; x < width; x++) {
      const ci: Uint8 = srcData[colorTableSize + y * srcStride + x];
      // TODO: Check how to handle out-of-bounds color indexes (currently we default to opaque black)
      const c: Uint32 = ci < colors.length ? colors[ci] : 0x000000ff;
      dataView.setUint32(RGBA_SIZE * (y * width + x), c, false);
    }
  }
  const stride: UintSize = width * RGBA_SIZE;
  return {width, height, stride, data};
}
