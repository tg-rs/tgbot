searchState.loadedDescShard("rustls_pki_types", 0, "This crate provides types for representing X.509 …\nFailure to parse an IP address\nA DER encoding of the PKIX AlgorithmIdentifier type:\nA DER-encoded X.509 certificate; as specified in RFC 5280\nA Certificate Revocation List; as specified in RFC 5280\nA Certificate Signing Request; as specified in RFC 2986\nDER-encoded data, either owned or borrowed\nA type which encapsulates a string (borrowed or owned) …\nThe server is identified by a DNS name.  The name is sent …\nA TLS-encoded Encrypted Client Hello (ECH) configuration …\nThe provided input could not be parsed because it is not a …\nA detail-less error when a signature is not valid.\n<code>no_std</code> implementation of <code>std::net::IpAddr</code>.\nThe server is identified by an IP address. SNI is not done.\n<code>no_std</code> implementation of <code>std::net::Ipv4Addr</code>.\n<code>no_std</code> implementation of <code>std::net::Ipv6Addr</code>.\nAn RSA private key\nA PKCS#8 private key\nA DER-encoded X.509 private key, in one of several formats\nA DER-encoded plaintext RSA private key; as specified in …\nA DER-encoded plaintext private key; as specified in …\nA Sec1-encoded plaintext private key; as specified in RFC …\nA Sec1 private key\nEncodes ways a client can know the expected name of the …\nAn abstract signature verification algorithm.\nA DER-encoded SubjectPublicKeyInfo (SPKI), as specified in …\nA DER-encoded SubjectPublicKeyInfo (SPKI), as specified in …\nA trust anchor (a.k.a. root CA)\nA timestamp, tracking the number of non-leap seconds since …\nAn Ipv4 address.\nAn Ipv6 address.\nNumber of seconds since the Unix epoch\nProduce a borrowed <code>DnsName</code> from this owned <code>DnsName</code>.\nClone the private key to a <code>&#39;static</code> value\nClone the private key to a <code>&#39;static</code> value\nClone the private key to a <code>&#39;static</code> value\nClone the private key to a <code>&#39;static</code> value\nConvert an iterator over PEM items into an …\nReturn <code>true</code> if this is backed by a FIPS-approved …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nA const constructor to create a <code>CertificateDer</code> from a …\nMakes a new <code>AlgorithmIdentifier</code> from a static octet slice.\nA const constructor to create a <code>Der</code> from a borrowed slice\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts this certificate into its owned variant, …\nConverts this SubjectPublicKeyInfo into its owned variant, …\nConverts this config into its owned variant, unfreezing …\nValue of DER-encoded <code>NameConstraints</code>, containing name …\nThe current time, as a <code>UnixTime</code>\nLow-level PEM decoding APIs.\nReturn the <code>AlgorithmIdentifier</code> that must equal a public key…\nYield the DER-encoded bytes of the private key\nYield the DER-encoded bytes of the private key\nYield the DER-encoded bytes of the private key\nYield the DER-encoded bytes of the private key\nReturn the <code>AlgorithmIdentifier</code> that must equal the …\nConvert a <code>Duration</code> since the start of 1970 to a <code>UnixTime</code>\nValue of the <code>subject</code> field of the trust anchor\nValue of the <code>subjectPublicKeyInfo</code> field of the trust anchor\nCopy this object to produce an owned <code>DnsName</code>, smashing the …\nProduce an owned <code>ServerName</code> from this (potentially …\nProduce an owned <code>DnsName</code> from this (potentially borrowed) …\nYield a <code>&#39;static</code> lifetime of the <code>TrustAnchor</code> by allocating …\nReturn the string representation of this <code>ServerName</code>.\nVerify a signature.\nbase64 decode error\nA DER-encoded x509 certificate.\nA Certificate Revocation List; as specified in RFC 5280\nA Certificate Signing Request; as specified in RFC 2986\nA Sec1-encoded plaintext private key; as specified in RFC …\nAn EchConfigList structure, as specified in …\nErrors that may arise when parsing the contents of a PEM …\nsyntax error found in the line that starts a new section\nI/O errors, from APIs that accept <code>std::io</code> types.\na section is missing its “END marker” line\nNo items found of desired type\nItems that can be decoded from PEM data.\nA DER-encoded plaintext private key; as specified in PKCS …\nA DER-encoded Subject Public Key Info; as specified in RFC …\nExtract and return all PEM sections by reading <code>rd</code>.\nA DER-encoded plaintext RSA private key; as specified in …\nA single recognised section in a PEM file.\nIterator over all PEM sections in a <code>&amp;[u8]</code> slice.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nExtract and decode the next supported PEM section from <code>rd</code>.\nConversion from a PEM <code>SectionKind</code> and body data.\nDecode the first section of this type from the PEM …\nDecode the first section of this type from PEM read from …\nDecode the first section of this type from PEM contained in\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new iterator.\nCreate a new iterator.\nIterate over all sections of this type from the PEM …\nIterate over all sections of this type from PEM present in …\nIterate over all sections of this type from PEM contained …\nthe expected “END marker” line that was not found\nline that contains the syntax error")