/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://w3c.github.io/webauthn/
 */

/***** Interfaces to Data *****/

[SecureContext, Pref="security.webauth.webauthn"]
interface PublicKeyCredential : Credential {
    [SameObject] readonly attribute ArrayBuffer              rawId;
    [SameObject] readonly attribute AuthenticatorResponse    response;
    AuthenticationExtensionsClientOutputs getClientExtensionResults();
};

partial dictionary CredentialCreationOptions {
    PublicKeyCredentialCreationOptions      publicKey;
};

partial dictionary CredentialRequestOptions {
    PublicKeyCredentialRequestOptions      publicKey;
};

[SecureContext]
partial interface PublicKeyCredential {
    static Promise<boolean> isUserVerifyingPlatformAuthenticatorAvailable();
};

[SecureContext, Pref="security.webauth.webauthn"]
interface AuthenticatorResponse {
    [SameObject] readonly attribute ArrayBuffer clientDataJSON;
};

[SecureContext, Pref="security.webauth.webauthn"]
interface AuthenticatorAttestationResponse : AuthenticatorResponse {
    [SameObject] readonly attribute ArrayBuffer      attestationObject;
    sequence<DOMString>                              getTransports();
    [Throws] ArrayBuffer                             getAuthenticatorData();
    [Throws] ArrayBuffer?                            getPublicKey();
    [Throws] COSEAlgorithmIdentifier                 getPublicKeyAlgorithm();
};

[SecureContext, Pref="security.webauth.webauthn"]
interface AuthenticatorAssertionResponse : AuthenticatorResponse {
    [SameObject] readonly attribute ArrayBuffer      authenticatorData;
    [SameObject] readonly attribute ArrayBuffer      signature;
    [SameObject] readonly attribute ArrayBuffer?     userHandle;
};

dictionary PublicKeyCredentialParameters {
    // Should be `DOMString`.
    required PublicKeyCredentialType  type;
    required COSEAlgorithmIdentifier  alg;
};

dictionary PublicKeyCredentialCreationOptions {
    required PublicKeyCredentialRpEntity   rp;
    required PublicKeyCredentialUserEntity user;

    required BufferSource                            challenge;
    required sequence<PublicKeyCredentialParameters> pubKeyCredParams;

    unsigned long                                timeout;
    sequence<PublicKeyCredentialDescriptor>      excludeCredentials = [];
    AuthenticatorSelectionCriteria               authenticatorSelection;
    // Should be `DOMString`.
    AttestationConveyancePreference              attestation = "none";
    AuthenticationExtensionsClientInputs         extensions;
};

dictionary PublicKeyCredentialEntity {
    required DOMString    name;
    [RustDeprecated="Removed from the WebAuthn specification"]
    USVString             icon;
};

dictionary PublicKeyCredentialRpEntity : PublicKeyCredentialEntity {
    DOMString      id;
};

dictionary PublicKeyCredentialUserEntity : PublicKeyCredentialEntity {
    required BufferSource   id;
    required DOMString      displayName;
};

dictionary AuthenticatorSelectionCriteria {
    // Should be `DOMString`.
    AuthenticatorAttachment      authenticatorAttachment;
    DOMString                    residentKey;
    boolean                      requireResidentKey = false;
    // Should be `DOMString`.
    UserVerificationRequirement  userVerification = "preferred";
};

enum AuthenticatorAttachment {
    "platform",       // Platform attachment
    "cross-platform"  // Cross-platform attachment
};

enum ResidentKeyRequirement {
    "discouraged",
    "preferred",
    "required"
};

enum AttestationConveyancePreference {
    "none",
    "indirect",
    "direct",
    "enterprise"
};

dictionary PublicKeyCredentialRequestOptions {
    required BufferSource                challenge;
    unsigned long                        timeout;
    USVString                            rpId;
    sequence<PublicKeyCredentialDescriptor> allowCredentials = [];
    // Should be `DOMString`.
    UserVerificationRequirement          userVerification = "preferred";
    AuthenticationExtensionsClientInputs extensions;
};

dictionary AuthenticationExtensionsClientInputs {
};

dictionary AuthenticationExtensionsClientOutputs {
};

dictionary CollectedClientData {
    required DOMString           type;
    required DOMString           challenge;
    required DOMString           origin;
    [RustDeprecated="Removed from the WebAuthn specification"]
    required DOMString           hashAlgorithm;
    [RustDeprecated="Removed from the WebAuthn specification"]
    DOMString                    tokenBindingId;
    [RustDeprecated="Removed from the WebAuthn specification"]
    AuthenticationExtensionsClientInputs clientExtensions;
    [RustDeprecated="Removed from the WebAuthn specification"]
    AuthenticationExtensionsAuthenticatorInputs authenticatorExtensions;
    boolean                      crossOrigin;
    TokenBinding                 tokenBinding;
};

dictionary TokenBinding {
    required DOMString status;
    DOMString id;
};

enum TokenBindingStatus { "present", "supported" };

enum PublicKeyCredentialType {
    "public-key"
};

dictionary PublicKeyCredentialDescriptor {
    // Should be `DOMString`.
    required PublicKeyCredentialType      type;
    required BufferSource                 id;
    // Should be `DOMString`.
    sequence<AuthenticatorTransport>      transports;
};

enum AuthenticatorTransport {
    "usb",
    "nfc",
    "ble",
    "internal"
};

typedef long COSEAlgorithmIdentifier;

enum UserVerificationRequirement {
    "required",
    "preferred",
    "discouraged"
};

partial dictionary AuthenticationExtensionsClientInputs {
    // FIDO AppID Extension (appid)
    // <https://w3c.github.io/webauthn/#sctn-appid-extension>
    USVString appid;
};

partial dictionary AuthenticationExtensionsClientOutputs {
    // FIDO AppID Extension (appid)
    // <https://w3c.github.io/webauthn/#sctn-appid-extension>
    boolean appid;
};

partial dictionary AuthenticationExtensionsClientInputs {
  USVString appidExclude;
};

partial dictionary AuthenticationExtensionsClientOutputs {
  boolean appidExclude;
};

partial dictionary AuthenticationExtensionsClientInputs {
  boolean uvm;
};

typedef sequence<unsigned long> UvmEntry;
typedef sequence<UvmEntry> UvmEntries;

partial dictionary AuthenticationExtensionsClientOutputs {
  UvmEntries uvm;
};

partial dictionary AuthenticationExtensionsClientInputs {
    boolean credProps;
};

dictionary CredentialPropertiesOutput {
    boolean rk;
};

partial dictionary AuthenticationExtensionsClientOutputs {
    CredentialPropertiesOutput credProps;
};

partial dictionary AuthenticationExtensionsClientInputs {
    AuthenticationExtensionsLargeBlobInputs largeBlob;
};

enum LargeBlobSupport {
  "required",
  "preferred",
};

dictionary AuthenticationExtensionsLargeBlobInputs {
    DOMString support;
    boolean read;
    BufferSource write;
};

partial dictionary AuthenticationExtensionsClientOutputs {
    AuthenticationExtensionsLargeBlobOutputs largeBlob;
};

dictionary AuthenticationExtensionsLargeBlobOutputs {
    boolean supported;
    ArrayBuffer blob;
    boolean written;
};
