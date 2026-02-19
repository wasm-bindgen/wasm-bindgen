/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://www.w3.org/TR/image-capture/
 *
 * Copyright © 2012-2025 W3C® (MIT, ERCIM, Keio, Beihang), All Rights Reserved.
 * W3C liability, trademark and document use rules apply.
 */

[Exposed=Window, SecureContext]
interface ImageCapture {
   constructor(MediaStreamTrack videoTrack);
   Promise<Blob>              takePhoto(optional PhotoSettings photoSettings = {});
   Promise<PhotoCapabilities> getPhotoCapabilities();
   Promise<PhotoSettings>     getPhotoSettings();

   Promise<ImageBitmap>       grabFrame();

   readonly attribute MediaStreamTrack track;
};

dictionary PhotoCapabilities {
  RedEyeReduction         redEyeReduction;
  MediaSettingsRange      imageHeight;
  MediaSettingsRange      imageWidth;
  sequence<FillLightMode> fillLightMode;
};

dictionary PhotoSettings {
  FillLightMode   fillLightMode;
  double          imageHeight;
  double          imageWidth;
  boolean         redEyeReduction;
};

dictionary MediaSettingsRange {
    double max;
    double min;
    double step;
};

enum RedEyeReduction {
  "never",
  "always",
  "controllable"
};

enum FillLightMode {
  "auto",
  "off",
  "flash"
};

enum MeteringMode {
  "none",
  "manual",
  "single-shot",
  "continuous"
};

dictionary Point2D {
  double x = 0.0;
  double y = 0.0;
};
