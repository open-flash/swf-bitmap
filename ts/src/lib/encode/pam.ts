import { TextEncoder } from "util";
import { SwfBitmap } from "../image";

const UTF8_ENCODER: TextEncoder = new TextEncoder();

/**
 * Export an image data object to the PAM format.
 *
 * Used internally for tests.
 *
 * Media type: image/x-portable-arbitrarymap
 *
 * @see http://netpbm.sourceforge.net/doc/pam.html
 * @param image Image data to export
 * @return The export PAM buffer
 * @internal
 */
export function encodePam(image: SwfBitmap): Uint8Array {
  const headerParts: string[] = [];
  headerParts.push("P7");
  headerParts.push(`WIDTH ${image.width.toString(10)}`);
  headerParts.push(`HEIGHT ${image.height.toString(10)}`);
  headerParts.push("DEPTH 4");
  headerParts.push("MAXVAL 255");
  headerParts.push("TUPLTYPE RGB_ALPHA");
  headerParts.push("ENDHDR");
  headerParts.push("");
  const headerStr: string = headerParts.join("\n");

  const header: Uint8Array = UTF8_ENCODER.encode(headerStr);

  const result: Uint8Array = new Uint8Array(header.length + image.data.length);
  result.set(header, 0);
  result.set(image.data, header.length);
  return result;
}
