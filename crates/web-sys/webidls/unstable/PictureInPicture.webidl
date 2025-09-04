/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://www.w3.org/TR/picture-in-picture/#idl-index
 */

partial interface HTMLVideoElement {
  [NewObject] Promise<PictureInPictureWindow> requestPictureInPicture();

  attribute EventHandler onenterpictureinpicture;
  attribute EventHandler onleavepictureinpicture;

  [CEReactions] attribute boolean disablePictureInPicture;
};

partial interface Document {
  readonly attribute boolean pictureInPictureEnabled;

  [NewObject] Promise<undefined> exitPictureInPicture();
};

partial interface mixin DocumentOrShadowRoot {
  readonly attribute Element? pictureInPictureElement;
};

[Exposed=Window]
interface PictureInPictureWindow : EventTarget {
  readonly attribute long width;
  readonly attribute long height;

  attribute EventHandler onresize;
};

[Exposed=Window]
interface PictureInPictureEvent : Event {
    constructor(DOMString type, PictureInPictureEventInit eventInitDict);
    [SameObject] readonly attribute PictureInPictureWindow pictureInPictureWindow;
};

dictionary PictureInPictureEventInit : EventInit {
    required PictureInPictureWindow pictureInPictureWindow;
};