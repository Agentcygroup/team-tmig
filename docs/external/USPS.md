# USPS ZIP+4 Retrieval

Authority: United States Postal Service
Host: postalpro.usps.com (also usps.com/business/web-tools-apis)
Endpoints:
  https://postalpro.usps.com/ZIP4 (registration required)
  https://postalpro.usps.com/address-quality/zip4
  https://www.usps.com/business/web-tools-apis/

Access: registration required for the full quarterly file
Free trial: USPS Web Tools API provides a limited lookup
Commercial: third-party data providers (Melissa, SmartyStreets, Lob) offer API access

Failure observed: 99130-byte response, which is a redirect stub, not the full file

Diagnosis: the placeholder URL in the prior script was not the actual download URL.
The real ZIP+4 file is behind an authenticated session at postalpro.usps.com.
