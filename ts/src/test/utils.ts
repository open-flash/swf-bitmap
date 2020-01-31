import { UintSize } from "semantic-types";

export function prettyPrintBytes(bytes: Uint8Array): string {
  const lines: string[] = [];
  for (let i: UintSize = 0; i < bytes.length; i += 16) {
    const slice: Uint8Array = bytes.slice(i, i + 16);
    const sliceHex: string[] = [];
    for (let j: UintSize = 0; j < slice.length; j += 4) {
      sliceHex.push(Buffer.from(slice.slice(j, j + 4)).toString("hex"));
    }
    lines.push(sliceHex.join(" "));
  }
  return `${lines.join("\n")}\n`;
}
