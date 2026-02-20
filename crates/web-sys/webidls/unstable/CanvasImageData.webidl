/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/*
 * Canvas 2D Context - CanvasImageData mixin
 *
 * The origin of this IDL file is:
 * https://html.spec.whatwg.org/multipage/canvas.html#canvasimagedata
 *
 * Per the spec, getImageData and putImageData should use `long` (i32) for
 * coordinates, not `double` (f64). This corrects the stable API.
 * See https://github.com/nicksenger/wasm-bindgen/pull/1920
 */

partial interface mixin CanvasImageData {
  [NewObject, Throws]
  ImageData getImageData(long sx, long sy, long sw, long sh);
  [Throws]
  undefined putImageData(ImageData imagedata, long dx, long dy);
  [Throws]
  undefined putImageData(ImageData imagedata, long dx, long dy, long dirtyX, long dirtyY, long dirtyWidth, long dirtyHeight);
};
