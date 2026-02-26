// https://w3c.github.io/webcodecs/#idl-index
// 29 January 2026

[Exposed=(Window,DedicatedWorker), SecureContext]
interface AudioDecoder : EventTarget {
  constructor(AudioDecoderInit init);

  readonly attribute CodecState state;
  readonly attribute unsigned long decodeQueueSize;
  attribute EventHandler ondequeue;

  [Throws] undefined configure(AudioDecoderConfig config);
  [Throws] undefined decode(EncodedAudioChunk chunk);
  Promise<undefined> flush();
  [Throws] undefined reset();
  [Throws] undefined close();

  static Promise<AudioDecoderSupport> isConfigSupported(AudioDecoderConfig config);
};

dictionary AudioDecoderInit {
  required AudioDataOutputCallback output;
  required WebCodecsErrorCallback error;
};

callback AudioDataOutputCallback = undefined(AudioData output);

[Exposed=(Window,DedicatedWorker), SecureContext]
interface VideoDecoder : EventTarget {
  constructor(VideoDecoderInit init);

  readonly attribute CodecState state;
  readonly attribute unsigned long decodeQueueSize;
  attribute EventHandler ondequeue;

  [Throws] undefined configure(VideoDecoderConfig config);
  [Throws] undefined decode(EncodedVideoChunk chunk);
  Promise<undefined> flush();
  [Throws] undefined reset();
  [Throws] undefined close();

  static Promise<VideoDecoderSupport> isConfigSupported(VideoDecoderConfig config);
};

dictionary VideoDecoderInit {
  required VideoFrameOutputCallback output;
  required WebCodecsErrorCallback error;
};

callback VideoFrameOutputCallback = undefined(VideoFrame output);

[Exposed=(Window,DedicatedWorker), SecureContext]
interface AudioEncoder : EventTarget {
  constructor(AudioEncoderInit init);

  readonly attribute CodecState state;
  readonly attribute unsigned long encodeQueueSize;
  attribute EventHandler ondequeue;

  [Throws] undefined configure(AudioEncoderConfig config);
  [Throws] undefined encode(AudioData data);
  Promise<undefined> flush();
  [Throws] undefined reset();
  [Throws] undefined close();

  static Promise<AudioEncoderSupport> isConfigSupported(AudioEncoderConfig config);
};

dictionary AudioEncoderInit {
  required EncodedAudioChunkOutputCallback output;
  required WebCodecsErrorCallback error;
};

callback EncodedAudioChunkOutputCallback =
    undefined (EncodedAudioChunk output,
               optional EncodedAudioChunkMetadata metadata = {});

dictionary EncodedAudioChunkMetadata {
  AudioDecoderConfig decoderConfig;
};

[Exposed=(Window,DedicatedWorker), SecureContext]
interface VideoEncoder : EventTarget {
  constructor(VideoEncoderInit init);

  readonly attribute CodecState state;
  readonly attribute unsigned long encodeQueueSize;
  attribute EventHandler ondequeue;

  [Throws] undefined configure(VideoEncoderConfig config);
  [Throws] undefined encode(VideoFrame frame, optional VideoEncoderEncodeOptions options = {});
  Promise<undefined> flush();
  [Throws] undefined reset();
  [Throws] undefined close();

  static Promise<VideoEncoderSupport> isConfigSupported(VideoEncoderConfig config);
};

dictionary VideoEncoderInit {
  required EncodedVideoChunkOutputCallback output;
  required WebCodecsErrorCallback error;
};

callback EncodedVideoChunkOutputCallback =
    undefined (EncodedVideoChunk chunk,
               optional EncodedVideoChunkMetadata metadata = {});

dictionary EncodedVideoChunkMetadata {
  VideoDecoderConfig decoderConfig;
  SvcOutputMetadata svc;
  BufferSource alphaSideData;
};

dictionary SvcOutputMetadata {
  unsigned long temporalLayerId;
};

dictionary AudioDecoderSupport {
  boolean supported;
  AudioDecoderConfig config;
};

dictionary VideoDecoderSupport {
  boolean supported;
  VideoDecoderConfig config;
};

dictionary AudioEncoderSupport {
  boolean supported;
  AudioEncoderConfig config;
};

dictionary VideoEncoderSupport {
  boolean supported;
  VideoEncoderConfig config;
};

dictionary AudioDecoderConfig {
  required DOMString codec;
  [EnforceRange] required unsigned long sampleRate;
  [EnforceRange] required unsigned long numberOfChannels;
  AllowSharedBufferSource description;
};

dictionary VideoDecoderConfig {
  required DOMString codec;
  AllowSharedBufferSource description;
  [EnforceRange] unsigned long codedWidth;
  [EnforceRange] unsigned long codedHeight;
  [EnforceRange] unsigned long displayAspectWidth;
  [EnforceRange] unsigned long displayAspectHeight;
  VideoColorSpaceInit colorSpace;
  HardwareAcceleration hardwareAcceleration = "no-preference";
  boolean optimizeForLatency;
  double rotation = 0;
  boolean flip = false;
};

dictionary AudioEncoderConfig {
  required DOMString codec;
  [EnforceRange] required unsigned long sampleRate;
  [EnforceRange] required unsigned long numberOfChannels;
  [EnforceRange] unsigned long long bitrate;
  BitrateMode bitrateMode = "variable";
};

dictionary VideoEncoderConfig {
  required DOMString codec;
  [EnforceRange] required unsigned long width;
  [EnforceRange] required unsigned long height;
  [EnforceRange] unsigned long displayWidth;
  [EnforceRange] unsigned long displayHeight;
  [EnforceRange] unsigned long long bitrate;
  double framerate;
  HardwareAcceleration hardwareAcceleration = "no-preference";
  AlphaOption alpha = "discard";
  DOMString scalabilityMode;
  VideoEncoderBitrateMode bitrateMode = "variable";
  LatencyMode latencyMode = "quality";
  DOMString contentHint;
};

enum HardwareAcceleration {
  "no-preference",
  "prefer-hardware",
  "prefer-software",
};

enum LatencyMode {
  "quality",
  "realtime"
};

dictionary VideoEncoderEncodeOptions {
  boolean keyFrame = false;
};

enum VideoEncoderBitrateMode {
  "constant",
  "variable",
  "quantizer"
};

enum CodecState {
  "unconfigured",
  "configured",
  "closed"
};

callback WebCodecsErrorCallback = undefined(DOMException error);

[Exposed=(Window,DedicatedWorker), Serializable]
interface EncodedAudioChunk {
  [Throws] constructor(EncodedAudioChunkInit init);
  readonly attribute EncodedAudioChunkType type;
  readonly attribute long long timestamp;          // microseconds
  readonly attribute unsigned long long? duration; // microseconds
  readonly attribute unsigned long byteLength;

  [Throws] undefined copyTo(AllowSharedBufferSource destination);
};

dictionary EncodedAudioChunkInit {
  required EncodedAudioChunkType type;
  [EnforceRange] required long long timestamp;    // microseconds
  [EnforceRange] unsigned long long duration;     // microseconds
  required AllowSharedBufferSource data;
  sequence<ArrayBuffer> transfer = [];
};

enum EncodedAudioChunkType {
    "key",
    "delta",
};

[Exposed=(Window,DedicatedWorker), Serializable]
interface EncodedVideoChunk {
  [Throws] constructor(EncodedVideoChunkInit init);
  readonly attribute EncodedVideoChunkType type;
  readonly attribute long long timestamp;             // microseconds
  readonly attribute unsigned long long? duration;    // microseconds
  readonly attribute unsigned long byteLength;

  [Throws] undefined copyTo(AllowSharedBufferSource destination);
};

dictionary EncodedVideoChunkInit {
  required EncodedVideoChunkType type;
  [EnforceRange] required long long timestamp;        // microseconds
  [EnforceRange] unsigned long long duration;         // microseconds
  required AllowSharedBufferSource data;
  sequence<ArrayBuffer> transfer = [];
};

enum EncodedVideoChunkType {
    "key",
    "delta",
};

[Exposed=(Window,DedicatedWorker), Serializable, Transferable]
interface AudioData {
  [Throws] constructor(AudioDataInit init);

  readonly attribute AudioSampleFormat? format;
  readonly attribute float sampleRate;
  readonly attribute unsigned long numberOfFrames;
  readonly attribute unsigned long numberOfChannels;
  readonly attribute unsigned long long duration;  // microseconds
  readonly attribute long long timestamp;          // microseconds

  [Throws] unsigned long allocationSize(AudioDataCopyToOptions options);
  [Throws] undefined copyTo(AllowSharedBufferSource destination, AudioDataCopyToOptions options);
  [Throws] AudioData clone();
  undefined close();
};

dictionary AudioDataInit {
  required AudioSampleFormat format;
  required float sampleRate;
  [EnforceRange] required unsigned long numberOfFrames;
  [EnforceRange] required unsigned long numberOfChannels;
  [EnforceRange] required long long timestamp;  // microseconds
  required BufferSource data;
  sequence<ArrayBuffer> transfer = [];
};

dictionary AudioDataCopyToOptions {
  [EnforceRange] required unsigned long planeIndex;
  [EnforceRange] unsigned long frameOffset = 0;
  [EnforceRange] unsigned long frameCount;
  AudioSampleFormat format;
};

enum AudioSampleFormat {
  "u8",
  "s16",
  "s32",
  "f32",
  "u8-planar",
  "s16-planar",
  "s32-planar",
  "f32-planar",
};

[Exposed=(Window,DedicatedWorker), SecureContext]
interface ImageDecoder {
  [Throws] constructor(ImageDecoderInit init);

  readonly attribute DOMString type;
  readonly attribute boolean complete;
  readonly attribute Promise<undefined> completed;
  readonly attribute ImageTrackList tracks;

  Promise<ImageDecodeResult> decode(optional ImageDecodeOptions options = {});
  undefined reset();
  undefined close();

  static Promise<boolean> isTypeSupported(DOMString type);
};


typedef (AllowSharedBufferSource or ReadableStream) ImageBufferSource;
dictionary ImageDecoderInit {
  required DOMString type;
  required ImageBufferSource data;
  ColorSpaceConversion colorSpaceConversion = "default";
  [EnforceRange] unsigned long desiredWidth;
  [EnforceRange] unsigned long desiredHeight;
  boolean preferAnimation;
  sequence<ArrayBuffer> transfer = [];
};


dictionary ImageDecodeOptions {
  [EnforceRange] unsigned long frameIndex = 0;
  boolean completeFramesOnly = true;
};


dictionary ImageDecodeResult {
  required VideoFrame image;
  required boolean complete;
};


[Exposed=(Window,DedicatedWorker), SecureContext]
interface ImageTrackList {
  getter ImageTrack (unsigned long index);

  readonly attribute Promise<undefined> ready;
  readonly attribute unsigned long length;
  readonly attribute long selectedIndex;
  readonly attribute ImageTrack? selectedTrack;
};


[Exposed=(Window,DedicatedWorker), SecureContext]
interface ImageTrack {
  readonly attribute boolean animated;
  readonly attribute unsigned long frameCount;
  readonly attribute unrestricted float repetitionCount;
  attribute boolean selected;
};

[Exposed=(Window,DedicatedWorker), Serializable, Transferable]
partial interface VideoFrame {
  readonly attribute double rotation;
  readonly attribute boolean flip;

  VideoFrameMetadata metadata();
};

dictionary VideoFrameMetadata {
  // Possible members are recorded in the VideoFrame Metadata Registry.
};

partial dictionary VideoFrameInit {
  double rotation = 0;
  boolean flip = false;

  VideoFrameMetadata metadata;
};

partial dictionary VideoFrameBufferInit {
  double rotation = 0;
  boolean flip = false;

  sequence<ArrayBuffer> transfer = [];

  VideoFrameMetadata metadata;
};
