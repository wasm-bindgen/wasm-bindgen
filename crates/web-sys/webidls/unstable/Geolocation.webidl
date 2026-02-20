/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://www.w3.org/TR/geolocation
 *
 * Copyright © 2021 W3C® (MIT, ERCIM, Keio, Beihang). W3C
 * liability, trademark and permissive document license rules apply.
 */

// Unstable callbacks using the new GeolocationPosition/GeolocationPositionError types
callback GeolocationPositionCallback = undefined (
  GeolocationPosition position
);

callback GeolocationPositionErrorCallback = undefined (
  GeolocationPositionError positionError
);

// Partial interface adding methods that use the new callback types
[Exposed=Window]
partial interface Geolocation {
  undefined getCurrentPosition (
    GeolocationPositionCallback successCallback,
    optional GeolocationPositionErrorCallback? errorCallback = null,
    optional PositionOptions options = {}
  );

  long watchPosition (
    GeolocationPositionCallback successCallback,
    optional GeolocationPositionErrorCallback? errorCallback = null,
    optional PositionOptions options = {}
  );
};
