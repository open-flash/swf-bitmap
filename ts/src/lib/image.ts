import { UintSize } from "semantic-types";

export interface SwfBitmapMeta {
  /**
   * Width in pixels.
   */
  readonly width: UintSize;

  /**
   * Height in pixels.
   */
  readonly height: UintSize;

  /**
   * Bytes per row (stride >= width * bytes_per_pixel)
   */
  readonly stride: UintSize;
}

export interface SwfBitmap extends SwfBitmapMeta {
  /**
   * Returns the one-dimensional array containing the data in RGBA order, as integers in the range 0 to 255.
   */
  readonly data: Uint8ClampedArray;
}
