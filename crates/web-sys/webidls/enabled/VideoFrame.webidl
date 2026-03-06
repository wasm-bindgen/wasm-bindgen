// https://w3c.github.io/webcodecs/#idl-index
// 29 January 2026

enum AlphaOption {
  "keep",
  "discard",
};

[Exposed=(Window,DedicatedWorker), Serializable, Transferable, WbgGeneric]
interface VideoFrame {
  [Throws] constructor(CanvasImageSource image, optional VideoFrameInit init = {});
  [Throws] constructor(AllowSharedBufferSource data, VideoFrameBufferInit init);

  readonly attribute VideoPixelFormat? format;
  readonly attribute unsigned long codedWidth;
  readonly attribute unsigned long codedHeight;
  readonly attribute DOMRectReadOnly? codedRect;
  readonly attribute DOMRectReadOnly? visibleRect;
  // readonly attribute double rotation;
  // readonly attribute boolean flip;
  readonly attribute unsigned long displayWidth;
  readonly attribute unsigned long displayHeight;
  readonly attribute unsigned long long? duration;  // microseconds
  readonly attribute long long timestamp;           // microseconds
  readonly attribute VideoColorSpace colorSpace;

  // VideoFrameMetadata metadata();

  [Throws] unsigned long allocationSize(
      optional VideoFrameCopyToOptions options = {});
  Promise<sequence<PlaneLayout>> copyTo(
      AllowSharedBufferSource destination,
      optional VideoFrameCopyToOptions options = {});
  [Throws] VideoFrame clone();
  undefined close();
};

[WbgGeneric]
dictionary VideoFrameInit {
  unsigned long long duration;  // microseconds
  long long timestamp;          // microseconds
  AlphaOption alpha = "keep";

  // Default matches image. May be used to efficiently crop. Will trigger
  // new computation of displayWidth and displayHeight using image's pixel
  // aspect ratio unless an explicit displayWidth and displayHeight are given.
  DOMRectInit visibleRect;

  // double rotation = 0;
  // boolean flip = false;

  // Default matches image unless visibleRect is provided.
  [EnforceRange] unsigned long displayWidth;
  [EnforceRange] unsigned long displayHeight;

  // VideoFrameMetadata metadata;
};

[WbgGeneric]
dictionary VideoFrameBufferInit {
  required VideoPixelFormat format;
  required [EnforceRange] unsigned long codedWidth;
  required [EnforceRange] unsigned long codedHeight;
  required [EnforceRange] long long timestamp;  // microseconds
  [EnforceRange] unsigned long long duration;  // microseconds

  // Default layout is tightly-packed.
  sequence<PlaneLayout> layout;

  // Default visible rect is coded size positioned at (0,0)
  DOMRectInit visibleRect;

  // double rotation = 0;
  // boolean flip = false;

  // Default display dimensions match visibleRect.
  [EnforceRange] unsigned long displayWidth;
  [EnforceRange] unsigned long displayHeight;

  VideoColorSpaceInit colorSpace;

  // sequence<ArrayBuffer> transfer = [];

  // VideoFrameMetadata metadata;
};

[WbgGeneric]
dictionary VideoFrameCopyToOptions {
  DOMRectInit rect;
  sequence<PlaneLayout> layout;
  VideoPixelFormat format;
  PredefinedColorSpace colorSpace;
};

[WbgGeneric]
dictionary PlaneLayout {
  [EnforceRange] required unsigned long offset;
  [EnforceRange] required unsigned long stride;
};

enum VideoPixelFormat {
  // 4:2:0 Y, U, V
  "I420",
  "I420P10",
  "I420P12",
  // 4:2:0 Y, U, V, A
  "I420A",
  "I420AP10",
  "I420AP12",
  // 4:2:2 Y, U, V
  "I422",
  "I422P10",
  "I422P12",
  // 4:2:2 Y, U, V, A
  "I422A",
  "I422AP10",
  "I422AP12",
  // 4:4:4 Y, U, V
  "I444",
  "I444P10",
  "I444P12",
  // 4:4:4 Y, U, V, A
  "I444A",
  "I444AP10",
  "I444AP12",
  // 4:2:0 Y, UV
  "NV12",
  // 4:4:4 RGBA
  "RGBA",
  // 4:4:4 RGBX (opaque)
  "RGBX",
  // 4:4:4 BGRA
  "BGRA",
  // 4:4:4 BGRX (opaque)
  "BGRX",
};

[Exposed=(Window,DedicatedWorker), WbgGeneric]
interface VideoColorSpace {
  constructor(optional VideoColorSpaceInit init = {});

  readonly attribute VideoColorPrimaries? primaries;
  readonly attribute VideoTransferCharacteristics? transfer;
  readonly attribute VideoMatrixCoefficients? matrix;
  readonly attribute boolean? fullRange;

  [Default] VideoColorSpaceInit toJSON();
};

[WbgGeneric]
dictionary VideoColorSpaceInit {
  VideoColorPrimaries? primaries = null;
  VideoTransferCharacteristics? transfer = null;
  VideoMatrixCoefficients? matrix = null;
  boolean? fullRange = null;
};

enum VideoColorPrimaries {
  "bt709",
  "bt470bg",
  "smpte170m",
  "bt2020",
  "smpte432",
};

enum VideoTransferCharacteristics {
  "bt709",
  "smpte170m",
  "iec61966-2-1",
  "linear",
  "pq",
  "hlg",
};

enum VideoMatrixCoefficients {
  "rgb",
  "bt709",
  "bt470bg",
  "smpte170m",
  "bt2020-ncl",
};
