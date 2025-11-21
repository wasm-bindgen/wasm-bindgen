/* tslint:disable */
/* eslint-disable */
/**
 * The `MediaSourceEnum` enum.
 *
 * *This API requires the following crate features to be activated: `MediaSourceEnum`*
 */

type MediaSourceEnum = "camera" | "screen" | "application" | "window" | "browser" | "microphone" | "audioCapture" | "other";

export function get_media_source(): MediaSourceEnum;

export function get_url(): URL;
