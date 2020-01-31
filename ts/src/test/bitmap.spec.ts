import chai from "chai";
import * as fs from "fs";
import { JsonReader } from "kryo/readers/json";
import sysPath from "path";
import { $DefineBitmap, DefineBitmap } from "swf-types/tags";
import { decodeXSwfBmpSync } from "../lib/decode/x-swf-bmp";
import { encodePam } from "../lib/encode/pam";
import { SwfBitmap } from "../lib/image";
import meta from "./meta.js";
import { prettyPrintBytes } from "./utils";

const PROJECT_ROOT: string = sysPath.join(meta.dirname, "..", "..", "..");
const TEST_SAMPLES_ROOT: string = sysPath.join(PROJECT_ROOT, "..", "tests", "x-swf-bmp");
const JSON_READER: JsonReader = new JsonReader();

describe("bitmap", function () {
  it("x-swf-bmp", async function () {
    const inputJson: string = fs.readFileSync(
      sysPath.join(TEST_SAMPLES_ROOT, "homestuck-beta-3", "tag.json"),
      {encoding: "UTF-8"},
    );
    const bitmapTag: DefineBitmap = $DefineBitmap.read(JSON_READER, inputJson);
    const image: SwfBitmap = decodeXSwfBmpSync(bitmapTag.data);

    const actualPam: Uint8Array = encodePam(image);
    fs.writeFileSync(sysPath.join(TEST_SAMPLES_ROOT, "homestuck-beta-3", "local-actual.ts.pam"), actualPam);

    const expectedPam: Uint8Array = fs.readFileSync(
      sysPath.join(TEST_SAMPLES_ROOT, "homestuck-beta-3", "expected.pam"),
      {encoding: null},
    );

    try {
      chai.assert.deepEqual(actualPam, expectedPam);
    } catch (err) {
      chai.assert.strictEqual(prettyPrintBytes(actualPam), prettyPrintBytes(expectedPam));
      throw err;
    }
  });
});
