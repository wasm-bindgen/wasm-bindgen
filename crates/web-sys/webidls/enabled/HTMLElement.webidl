/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://www.whatwg.org/specs/web-apps/current-work/ and
 * http://dev.w3.org/csswg/cssom-view/
 *
 * © Copyright 2004-2011 Apple Computer, Inc., Mozilla Foundation, and
 * Opera Software ASA. You are granted a license to use, reproduce
 * and create derivative works of this document.
 */

[HTMLConstructor]
interface HTMLElement : Element {
  // metadata attributes
  [CEReactions]
           attribute DOMString title;
  [CEReactions]
           attribute long scrollHeight;
  [CEReactions]
           attribute long scrollTop;
  [CEReactions]
           attribute DOMString lang;
  //         attribute boolean translate;
  [CEReactions, SetterThrows, Pure]
           attribute DOMString dir;

  [CEReactions, GetterThrows, Pure, TreatNullAs=EmptyString]
           attribute DOMString innerText;

  // user interaction
  [CEReactions, SetterThrows, Pure]
           attribute boolean hidden;
  [CEReactions]
           attribute boolean inert;
  [NeedsCallerType]
  undefined click();
  [Throws]
  undefined focus(optional FocusOptions options = {});
  [Throws]
  undefined blur();
  [CEReactions, SetterThrows, Pure]
           attribute DOMString accessKey;
  [Pure]
  readonly attribute DOMString accessKeyLabel;
  [CEReactions, SetterThrows, Pure]
           attribute boolean draggable;
  //[PutForwards=value] readonly attribute DOMTokenList dropzone;
  [CEReactions, SetterThrows, Pure]
           attribute DOMString contentEditable;
  [Pure]
  readonly attribute boolean isContentEditable;
  //[Pure]
  //readonly attribute HTMLMenuElement? contextMenu;
  //[SetterThrows]
  //         attribute HTMLMenuElement? contextMenu;
  [CEReactions, SetterThrows, Pure, Pref="dom.element.popover.enabled"]
           attribute DOMString? popover;
  [CEReactions, SetterThrows, Pure]
           attribute boolean spellcheck;

  // command API
  //readonly attribute DOMString? commandType;
  //readonly attribute DOMString? commandLabel;
  //readonly attribute DOMString? commandIcon;
  //readonly attribute boolean? commandHidden;
  //readonly attribute boolean? commandDisabled;
  //readonly attribute boolean? commandChecked;

  [Throws, Pref="dom.element.popover.enabled"]
  undefined showPopover(optional ShowPopoverOptions options = {});
  [Throws, Pref="dom.element.popover.enabled"]
  undefined hidePopover();
  [Throws, Pref="dom.element.popover.enabled"]
  boolean togglePopover(optional boolean force);
};

// http://dev.w3.org/csswg/cssom-view/#extensions-to-the-htmlelement-interface
partial interface HTMLElement {
  // CSSOM things are not [Pure] because they can flush
  readonly attribute Element? offsetParent;
  readonly attribute long offsetTop;
  readonly attribute long offsetLeft;
  readonly attribute long offsetWidth;
  readonly attribute long offsetHeight;
};

// https://w3c.github.io/touch-events/#extensions-to-the-globaleventhandlers-mixin
interface mixin TouchEventHandlers {
  [Func="nsGenericHTMLElement::TouchEventsEnabled"]
           attribute EventHandler ontouchstart;
  [Func="nsGenericHTMLElement::TouchEventsEnabled"]
           attribute EventHandler ontouchend;
  [Func="nsGenericHTMLElement::TouchEventsEnabled"]
           attribute EventHandler ontouchmove;
  [Func="nsGenericHTMLElement::TouchEventsEnabled"]
           attribute EventHandler ontouchcancel;
};

HTMLElement includes HTMLOrSVGElement;
HTMLElement includes ElementCSSInlineStyle;
HTMLElement includes GlobalEventHandlers;
HTMLElement includes DocumentAndElementEventHandlers;
HTMLElement includes TouchEventHandlers;
HTMLElement includes OnErrorEventHandlerForNodes;

interface HTMLUnknownElement : HTMLElement {};
