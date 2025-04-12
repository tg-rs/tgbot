searchState.loadedDescShard("ring", 0, "Feature Flags\nAuthenticated Encryption with Associated Data (AEAD).\nKey Agreement: ECDH, including X25519.\nSHA-2 and the legacy SHA-1 digest algorithm.\nError reporting.\nHMAC-based Extract-and-Expand Key Derivation Function.\nHMAC is specified in RFC 2104.\nSerialization and deserialization.\nPBKDF2 derivation and verification.\nPKCS#8 is specified in RFC 5958.\nCryptographic pseudo-random number generation.\nRSA.\nPublic key signatures: signing and verification.\nReferences a test input file.\nAES-128 in GCM mode with 128-bit tags and 96 bit nonces.\nAES-256 in GCM mode with 128-bit tags and 96 bit nonces.\nThe additionally authenticated data (AAD) for an opening …\nAn AEAD Algorithm.\nAn AEAD key bound to a nonce sequence.\nChaCha20-Poly1305 as described in RFC 8439.\nImmutable keys for use in situations where <code>OpeningKey</code>/…\nThe maximum length of a tag for the algorithms in this …\nAll the AEADs we support use 96-bit nonces.\nA nonce for a single AEAD opening or sealing operation.\nA sequences of unique nonces.\nAn AEAD key for authenticating and decrypting (“opening…\nAn AEAD key for encrypting and signing (“sealing”), …\nA possibly valid authentication tag.\nAn AEAD key without a designated role or nonce sequence.\nReturns the next nonce in the sequence.\nThe key’s AEAD algorithm.\nThe key’s AEAD algorithm.\nThe key’s AEAD algorithm.\nConstructs a <code>Nonce</code> with the given value, assuming that the …\nThe chacha20-poly1305@openssh.com AEAD-ish construct.\nConstruct an empty <code>Aad</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstruct the <code>Aad</code> from the given bytes.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe length of the key.\nConstructs a new key from the given <code>UnboundKey</code> and …\nConstructs a <code>UnboundKey</code>.\nConstructs a <code>LessSafeKey</code>.\nThe length of the nonces.\nAuthenticates and decrypts (“opens”) data in place.\nLike <code>super::OpeningKey::open_in_place()</code>, except it accepts …\nLike open_in_place, except the authentication tag is …\nAuthenticates and decrypts (“opens”) data in place, …\nLike <code>super::OpeningKey::open_within()</code>, except it accepts an\nQUIC Header Protection.\nEncrypts and signs (“seals”) data in place, appending …\nLike <code>super::SealingKey::seal_in_place_append_tag()</code>, except …\nEncrypts and signs (“seals”) data in place.\nLike <code>super::SealingKey::seal_in_place_separate_tag()</code>, …\nThe length of a tag.\nConstructs a <code>Nonce</code> with the given value, assuming that the …\nThe length of key.\nA key for opening packets.\nThe length in bytes of the <code>packet_length</code> field in a SSH …\nA key for sealing packets.\nThe length in bytes of an authentication tag.\nReturns the decrypted, but unauthenticated, packet length.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstructs a new <code>SealingKey</code>.\nConstructs a new <code>OpeningKey</code>.\nOpens (authenticates and decrypts) a packet.\nSeals (encrypts and signs) a packet.\nAES-128.\nAES-256.\nA QUIC Header Protection Algorithm.\nChaCha20.\nA key for generating QUIC Header Protection masks.\nQUIC sample for new key masks\nThe key’s algorithm.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe length of the key.\nCreate a new header protection key.\nGenerate a new QUIC Header Protection mask.\nThe required sample length.\nA key agreement algorithm.\nECDH using the NSA Suite B P-256 (secp256r1) curve.\nECDH using the NSA Suite B P-384 (secp384r1) curve.\nAn ephemeral private key for use (only) with …\nA public key for key agreement.\nAn unparsed, possibly malformed, public key for key …\nX25519 (ECDH using Curve25519) as described in RFC 7748.\nPerforms a key agreement with an ephemeral private key and …\nThe algorithm for the private key.\nThe algorithm for the public key.\nThe algorithm for the public key.\nTODO: doc\nComputes the public key from the private key.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGenerate a new ephemeral private key for the given …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstructs a new <code>UnparsedPublicKey</code>.\nA digest algorithm.\nA context for multi-step (Init-Update-Finish) digest …\nA calculated digest value.\nThe maximum block length (<code>Algorithm::block_len()</code>) of all …\nThe maximum chaining length (<code>Algorithm::chaining_len()</code>) of …\nThe maximum output length (<code>Algorithm::output_len()</code>) of all …\nSHA-1 as specified in FIPS 180-4. Deprecated.\nThe length of the output of SHA-1, in bytes.\nSHA-256 as specified in FIPS 180-4.\nThe length of the output of SHA-256, in bytes.\nSHA-384 as specified in FIPS 180-4.\nThe length of the output of SHA-384, in bytes.\nSHA-512 as specified in FIPS 180-4.\nSHA-512/256 as specified in FIPS 180-4.\nThe length of the output of SHA-512/256, in bytes.\nThe length of the output of SHA-512, in bytes.\nThe algorithm that this context is using.\nThe algorithm that was used to calculate the digest value.\nThe internal block length.\nThe size of the chaining value of the digest function, in …\nReturns the digest of <code>data</code> using the given digest …\nFinalizes the digest calculation and returns the digest …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstructs a new context.\nThe length of a finalized digest.\nUpdates the digest with all the data in <code>data</code>.\nAn error parsing or validating a key.\nAn error with absolutely no details.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAn HKDF algorithm.\nHKDF using HMAC-SHA-1. Obsolete.\nHKDF using HMAC-SHA-256.\nHKDF using HMAC-SHA-384.\nHKDF using HMAC-SHA-512.\nThe length of the OKM (Output Keying Material) for a …\nAn HKDF OKM (Output Keying Material)\nA HKDF PRK (pseudorandom key).\nA salt for HKDF operations.\nThe algorithm used to derive this salt.\nThe HKDF-Expand operation.\nThe HKDF-Extract operation.\nFills <code>out</code> with the output of the HKDF-Expand operation for …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe underlying HMAC algorithm.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe length that <code>Prk::expand()</code> should expand its input to.\nThe <code>OkmLength</code> given to <code>Prk::expand()</code>.\nConstructs a new <code>Salt</code> with the given value based on the …\nConstruct a new <code>Prk</code> directly with the given value.\nAn HMAC algorithm.\nA context for multi-step (Init-Update-Finish) HMAC signing.\nHMAC using SHA-1. Obsolete.\nHMAC using SHA-256.\nHMAC using SHA-384.\nHMAC using SHA-512.\nA key to use for HMAC signing.\nAn HMAC tag.\nThe digest algorithm for the key.\nThe digest algorithm this HMAC algorithm is based on.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGenerate an HMAC signing key using the given digest …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstruct an HMAC signing key using the given digest …\nCalculates the HMAC of <code>data</code> using the key <code>key</code> in one step.\nFinalizes the HMAC calculation and returns the HMAC value. …\nUpdates the HMAC with all the data in <code>data</code>. <code>update</code> may be …\nCalculates the HMAC of <code>data</code> using the signing key <code>key</code>, and …\nConstructs a new HMAC signing context using the given …\nA serialized positive integer.\nReturns the value, ordered from significant byte to least …\nReturns the first byte.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nA PBKDF2 algorithm.\nPBKDF2 using HMAC-SHA1.\nPBKDF2 using HMAC-SHA256.\nPBKDF2 using HMAC-SHA384.\nPBKDF2 using HMAC-SHA512.\nFills <code>out</code> with the key derived using PBKDF2 with the given …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nVerifies that a previously-derived (e.g., using <code>derive</code>) …\nA generated PKCS#8 document.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nA random value constructed from a <code>SecureRandom</code> that hasn’…\nA type that can be returned by <code>ring::rand::generate()</code>.\nA secure random number generator.\nA secure random number generator where the random values …\nExpose the random value.\nFills <code>dest</code> with random bytes.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGenerate the new random value using <code>rng</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstructs a new <code>SystemRandom</code>.\nAn RSA key pair, used for signing.\nRSA key pair components.\nAn RSA Public Key.\nRSA public key components.\nParameters for RSA verification.\nThe private exponent.\n<code>p</code>’s public Chinese Remainder Theorem exponent.\n<code>q</code>’s public Chinese Remainder Theorem exponent.\nThe public exponent, encoded in big-endian bytes without …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstructs an RSA private key from its big-endian-encoded …\nParses an RSA private key that is not inside a PKCS#8 …\nParses an unencrypted PKCS#8-encoded RSA private key.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe length, in bytes, of the public modulus.\nThe public modulus, encoded in big-endian bytes without …\nThe first prime factor of <code>d</code>.\nReturns a reference to the public key.\nThe public key components.\nReturns the length in bytes of the key pair’s public …\nThe second prime factor of <code>d</code>.\n<code>q**-1 mod p</code>.\nComputes the signature of <code>msg</code> and writes it into <code>signature</code>.\nVerifies that <code>signature</code> is a valid signature of <code>message</code> …\nVerification of ASN.1 DER-encoded ECDSA signatures using …\nSigning of ASN.1 DER-encoded ECDSA signatures using the …\nVerification of fixed-length (PKCS#11 style) ECDSA …\nSigning of fixed-length (PKCS#11 style) ECDSA signatures …\n<em>Not recommended</em>. Verification of ASN.1 DER-encoded ECDSA …\n<em>Not recommended</em>. Verification of ASN.1 DER-encoded ECDSA …\nVerification of ASN.1 DER-encoded ECDSA signatures using …\nSigning of ASN.1 DER-encoded ECDSA signatures using the …\nVerification of fixed-length (PKCS#11 style) ECDSA …\nSigning of fixed-length (PKCS#11 style) ECDSA signatures …\nVerification of Ed25519 signatures.\nThe length of an Ed25519 public key.\nAn ECDSA key pair, used for signing.\nAn ECDSA signing algorithm.\nAn ECDSA verification algorithm.\nAn Ed25519 key pair, for signing.\nParameters for EdDSA signing and verification.\nKey pairs for signing messages (private key and public …\nThe type of the public key.\nVerification of signatures using RSA keys of 1024-8192 …\nVerification of signatures using RSA keys of 1024-8192 …\nVerification of signatures using RSA keys of 1024-8192 …\nVerification of signatures using RSA keys of 2048-8192 …\nVerification of signatures using RSA keys of 2048-8192 …\nVerification of signatures using RSA keys of 2048-8192 …\nVerification of signatures using RSA keys of 2048-8192 …\nVerification of signatures using RSA keys of 3072-8192 …\nPKCS#1 1.5 padding using SHA-256 for RSA signatures.\nPKCS#1 1.5 padding using SHA-384 for RSA signatures.\nPKCS#1 1.5 padding using SHA-512 for RSA signatures.\nVerification of signatures using RSA keys of 2048-8192 …\nVerification of signatures using RSA keys of 2048-8192 …\nVerification of signatures using RSA keys of 2048-8192 …\nRSA PSS padding using SHA-256 for RSA signatures.\nRSA PSS padding using SHA-384 for RSA signatures.\nRSA PSS padding using SHA-512 for RSA signatures.\nAn RSA signature encoding as described in RFC 3447 Section …\nAn RSA key pair, used for signing.\nRSA public key components.\nA public key signature returned from a signing operation.\nAn unparsed, possibly malformed, public key for signature …\nA signature verification algorithm.\nThe public exponent, encoded in big-endian bytes without …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstructs an Ed25519 key pair by parsing an unencrypted …\nConstructs an ECDSA key pair by parsing an unencrypted …\nConstructs an Ed25519 key pair by parsing an unencrypted …\nConstructs an ECDSA key pair from the private key and …\nConstructs an Ed25519 key pair from the private key seed …\nConstructs a Ed25519 key pair from the private key seed …\nGenerates a new key pair and returns the key pair …\nGenerates a new key pair and returns the key pair …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe public modulus, encoded in big-endian bytes without …\nConstruct a new <code>UnparsedPublicKey</code>.\nThe public key for the key pair.\nReturns the signature of the message <code>msg</code>.\nReturns the signature of the <code>message</code> using a random nonce …\nVerify the signature <code>signature</code> of message <code>msg</code> with the …\nParses the public key and verifies <code>signature</code> is a valid …")